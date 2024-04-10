pub mod endpoints;
pub mod errors;
pub mod middleware;
pub mod transformer;
pub mod ws;

use crate::endpoints::hello;
use crate::endpoints::recognizer;
use crate::errors::WebError;
use crate::transformer::client::WhisperClient;

use actix_web::http::header;
use actix_web::{web, Scope};
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


pub fn build_swagger_service() -> SwaggerUi {
    let openapi = ApiDoc::openapi();
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone())
}
