use crate::errors::{ErrorResponse, SuccessfulResponse, WebError};
use crate::{ContextData, RecognizeData};
use crate::whisper::forms::{RecognizeParameters, RecognizeResponse};
use crate::whisper::helper;
use actix_multipart::Multipart;
use actix_web::{get, HttpResponse, post, web};
use actix_web::http::StatusCode;
use actix_web::web::Query;

#[utoipa::path(
    get,
    path = "/recognize/",
    tag = "Upload",
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
            status = 400,
            description = "Failed while getting upload form",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting upload form".to_string(),
            })
        ),
    ),
)]
#[get("/")]
pub async fn upload_form() -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/upload.html"))
}

#[utoipa::path(
    post,
    path = "/recognize/file",
    tag = "Recognize",
    params(
        (
            "concatenate", Query,
            description = "Concatenate chunked text to common",
            example = "true"
        )
    ),
    request_body(
        content_type = "multipart/formdata",
        content = Multipart,
        example = "To check open url /recognize/file into browser.",
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = [RecognizeResponse],
            example = json!([
                {
                    "frame_id": 0,
                    "frame_start": 0,
                    "frame_end": 3,
                    "text": "Hello",
                },
                {
                    "frame_id": 2,
                    "frame_start": 0,
                    "frame_end": 3,
                    "text": "world",
                }
            ])
        ),
        (
            status = 400,
            description = "Failed while recognizing audio file",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while recognizing audio file".to_string(),
            })
        ),
    )
)]
#[post("/file")]
pub async fn recognize_file(
    cxt: ContextData,
    concatenate: Query<bool>,
    payload: Multipart,
) -> RecognizeData<serde_json::Value> {
    let params = RecognizeParameters::default();
    match helper::extract_multiform_data(payload).await {
        Err(err) => Err(WebError::ResponseError(err.to_string())),
        Ok(file_path) => {
            let client = cxt.get_ref().get_client().await;
            match client.recognize_file(file_path.as_str(), &params).await {
                Err(err) => Err(WebError::ResponseError(err.to_string())),
                Ok(data) => {
                    if concatenate.0 {
                        let test = RecognizeResponse::from(data);
                        Ok(web::Json(serde_json::to_value(test).unwrap()))
                    } else {
                        Ok(web::Json(serde_json::to_value(data).unwrap()))
                    }
                },
            }
        },
    }
}


