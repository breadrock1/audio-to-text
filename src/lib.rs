pub mod endpoints;
pub mod errors;
pub mod middleware;
pub mod transformer;
pub mod ws;
mod swagger;

use crate::endpoints::hello;
use crate::endpoints::recognizer;
use crate::errors::WebError;
use crate::swagger::ApiDoc;
use crate::transformer::client::WhisperClient;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, Scope};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub type ContextData = web::Data<Box<WhisperClient>>;
pub type RecognizeData<T> = Result<web::Json<T>, WebError>;

pub fn build_hello_scope() -> Scope {
    web::scope("/hello").service(hello::hello)
}

pub fn build_recognize_scope() -> Scope {
    web::scope("/recognize")
        .service(recognizer::recognize_audio)
        .service(recognizer::recognize_audio_full_text)
        .service(recognizer::upload_file_form)
        .service(recognizer::recognize_audio_stream_form)
}

pub fn build_websocket_scope() -> Scope {
    let scope = web::scope("/ws");
    #[cfg(feature = "enable-streaming")]
    {
        use crate::ws::ws_streaming;
        scope.service(ws_streaming);
    }

    scope
}

pub fn build_cors_policy() -> Cors {
    let available_methods = vec!["GET", "POST", "OPTIONS"];
    let available_headers = vec![header::AUTHORIZATION, header::ACCEPT];

    Cors::default()
        .allowed_header(header::CONTENT_TYPE)
        .allowed_methods(available_methods)
        .allowed_headers(available_headers)
        .allow_any_origin()
        .max_age(3600)
}

pub fn build_swagger_service() -> SwaggerUi {
    let openapi = ApiDoc::openapi();
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone())
}
