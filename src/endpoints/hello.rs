use crate::errors::SuccessfulResponse;
use crate::transformer::client::WhisperClient;

use actix_web::web::Data;
use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn hello(cxt: Data<Box<WhisperClient>>) -> HttpResponse {
    let _client = cxt.get_ref();
    SuccessfulResponse::ok_response("Ok")
}
