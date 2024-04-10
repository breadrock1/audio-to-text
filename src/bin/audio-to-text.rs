extern crate audio_to_text;

use audio_to_text::*;
use audio_to_text::middleware::logger;
use audio_to_text::transformer::client::WhisperClient;
use audio_to_text::ws::ws_streaming;

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

    let model_path = std::env::var("WHISPER_MODEL_PATH")?;
    let enable_gpu_data = std::env::var("WHISPER_ENABLE_GPU")?;
    let enable_gpu = bool::from_str(enable_gpu_data.as_str())?;

    let whisper_context = WhisperClient::new(model_path.as_str(), enable_gpu);

    HttpServer::new(move || {
        let whisper_context = whisper_context.clone();
        let whisper_box_cxt: Box<WhisperClient> = Box::new(whisper_context);

        App::new()
            .app_data(web::Data::new(whisper_box_cxt))
            .wrap(Logger::default())
            .service(build_hello_scope())
            .service(build_recognize_scope())
            .service(build_swagger_service())
            .service(Files::new("/static", ".").show_files_listing())
    })
    .bind((service_host, service_port))?
    .run()
    .await?;

    Ok(())
}
