extern crate audio_to_text;

use audio_to_text::mws::{cors, logger};
use audio_to_text::healthcheck;
use audio_to_text::swagger;
use audio_to_text::whisper;
use audio_to_text::whisper::client_async::WhisperAsyncClient;
use audio_to_text::whisper::config::WhisperClientConfig;
use audio_to_text::ws;

use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use std::str::FromStr;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    #[cfg(feature = "enable-dotenv")]
    let _ = dotenv::dotenv();
    logger::build_env_logger();

    let service_host = std::env::var("SERVICE_HOST")?;
    let service_port_data = std::env::var("SERVICE_PORT")?;
    let service_port = u16::from_str(service_port_data.as_str())?;
    let workers = std::env::var("WORKERS_NUMBER")?;
    let workers_num = usize::from_str(workers.as_str())?;

    let whisper_config = WhisperClientConfig::from_env();
    let whisper_context = WhisperAsyncClient::new(&whisper_config);

    HttpServer::new(move || {
        let whisper_context = whisper_context.clone();
        let whisper_box_cxt: Box<WhisperAsyncClient> = Box::new(whisper_context);

        let static_files = Files::new("/static", ".");

        App::new()
            .app_data(web::Data::new(whisper_box_cxt))
            .wrap(Logger::default())
            .wrap(cors::build_cors_policy())
            .service(static_files.show_files_listing())
            .service(healthcheck::build_scope())
            .service(swagger::build_scope())
            .service(whisper::build_scope())
            .service(
                web::resource("/socket")
                    .route(web::get().to(ws::routes::stream))
            )
            .service(
                web::resource("/ws")
                    .route(web::get().to(ws::routes::websocket))
            )
    })
    .bind((service_host, service_port))?
    .workers(workers_num)
    .run()
    .await?;

    Ok(())
}
