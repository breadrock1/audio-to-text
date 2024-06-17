use crate::errors::{ErrorResponse, SuccessfulResponse};
use crate::whisper::client_async::WhisperAsyncClient;

use actix_web::{get, HttpResponse};
use actix_web::web::Data;

#[utoipa::path(
    get,
    path = "/hello/",
    tag = "Hello",
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Done".to_string(),
            })
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
            })
        )
    ),
)]
#[get("/")]
pub(crate) async fn check_health(cxt: Data<Box<WhisperAsyncClient>>) -> HttpResponse {
    let _client = cxt.get_ref();
    SuccessfulResponse::ok_response("Ok")
}
