mod hello;
mod recognizer;

use crate::endpoints::hello::hello as hello_endpoint;
use crate::endpoints::recognizer::*;
use crate::errors::WebError;
use crate::transformer::client::WhisperClient;

use actix_web::{web, Scope};

pub type ContextData = web::Data<Box<WhisperClient>>;
pub type RecognizeData<T> = Result<web::Json<T>, WebError>;

pub fn build_hello_scope() -> Scope {
    web::scope("/hello").service(hello_endpoint)
}

pub fn build_recognize_scope() -> Scope {
    web::scope("/recognize")
        .service(recognize_audio)
        .service(upload_file_form)
        .service(recognize_audio_stream_form)
}
