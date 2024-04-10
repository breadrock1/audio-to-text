use crate::endpoints::helper;
use crate::errors::WebError;
use crate::transformer::client::{RecognizeParameters, RecognizeResponse};
use crate::{ContextData, RecognizeData};

use actix_multipart::Multipart;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponse, Responder};

#[utoipa::path(
    get,
    path = "/recognize/file",
    tag = "Returns html page to upload file",
    responses(
        (status = 200, description = "Successful", body = SuccessfulResponse),
        (status = 400, description = "Server does not available", body = ErrorResponse),
    ),
)]
#[get("/file")]
pub async fn upload_file_form() -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/upload.html"))
}

#[get("/stream")]
pub async fn recognize_audio_stream_form(_cxt: ContextData) -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/stream.html"))
}

#[utoipa::path(
    post,
    path = "/recognize/file",
    tag = "Send audio file to recognize voice text data as with timeframes",
    request_body(
        content_type="multipart/formdata",
        content=Multipart,
    ),
    responses(
        (status = 200, description = "Successful", body = [RecognizeResponse]),
        (status = 400, description = "Failed while sending post request", body = ErrorResponse),
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
    tag = "Send audio file to recognize voice text data as common text",
    request_body(
        content_type="multipart/formdata",
        content=Multipart,
    ),
    responses(
        (status = 200, description = "Successful", body = RecognizeResponse),
        (status = 400, description = "Failed while sending post request", body = ErrorResponse),
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
