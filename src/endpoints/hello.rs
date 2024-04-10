use crate::errors::SuccessfulResponse;
use crate::transformer::client::WhisperClient;

use actix_web::web::Data;
use actix_web::{get, HttpResponse};

#[utoipa::path(
    get,
    path = "/hello/",
    tag = "Test server connection endpoint",
    responses(
        (status = 200, description = "Successful", body = SuccessfulResponse),
        (status = 501, description = "Server does not available", body = ErrorResponse),
    ),
)]
#[get("/")]
pub async fn hello(cxt: Data<Box<WhisperClient>>) -> HttpResponse {
    let _client = cxt.get_ref();
    SuccessfulResponse::ok_response("Ok")
}
