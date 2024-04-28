use crate::endpoints::helper;
use crate::errors::{ErrorResponse, SuccessfulResponse, WebError};
use crate::transformer::client::{RecognizeParameters, RecognizeResponse};
use crate::{ContextData, RecognizeData};

use actix_multipart::Multipart;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponse, Responder};

#[utoipa::path(
    get,
    path = "/recognize/file",
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
#[get("/file")]
pub async fn upload_file_form() -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/upload.html"))
}

#[utoipa::path(
    post,
    path = "/recognize/file",
    tag = "Recognize",
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
pub async fn recognize_audio(
    cxt: ContextData,
    payload: Multipart,
) -> RecognizeData<Vec<RecognizeResponse>> {
    let params = RecognizeParameters::default();
    match helper::extract_multiform_data(payload).await {
        Err(err) => Err(WebError::ResponseError(err.to_string())),
        Ok(file_path) => match cxt.get_ref().recognize(file_path.as_str(), params).await {
            Ok(data) => Ok(web::Json(data)),
            Err(err) => Err(WebError::ResponseError(err.to_string())),
        },
    }
}

#[utoipa::path(
    post,
    path = "/recognize/file-text",
    tag = "Recognize",
    request_body(
        content_type = "multipart/formdata",
        content = Multipart,
        example = "To check open url /recognize/file into browser.",
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = RecognizeResponse,
            example = json!([
                {
                    "frame_id": 0,
                    "frame_start": 0,
                    "frame_end": 10,
                    "text": "Hello world",
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
#[post("file-text")]
pub async fn recognize_audio_full_text(
    cxt: ContextData,
    payload: Multipart,
) -> RecognizeData<RecognizeResponse> {
    let params = RecognizeParameters::default();
    match helper::extract_multiform_data(payload).await {
        Err(err) => Err(WebError::ResponseError(err.to_string())),
        Ok(file_path) => match cxt.get_ref().recognize(file_path.as_str(), params).await {
            Ok(data) => Ok(web::Json(RecognizeResponse::from(data))),
            Err(err) => Err(WebError::ResponseError(err.to_string())),
        },
    }
}

#[get("/stream")]
pub async fn recognize_audio_stream_form(_cxt: ContextData) -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/stream.html"))
}
