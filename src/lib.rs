pub mod errors;
pub mod mws;
pub mod swagger;
pub mod ws;
pub mod healthcheck;
pub mod whisper;

use crate::errors::WebError;
use crate::whisper::client_async::WhisperAsyncClient;

use actix_web::web::{Data, Json};

pub type ContextData = Data<Box<WhisperAsyncClient>>;
pub type RecognizeData<T> = Result<Json<T>, WebError>;
