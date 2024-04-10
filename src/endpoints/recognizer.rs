use crate::endpoints::helper;
use crate::errors::WebError;
use crate::transformer::client::{RecognizeParameters, RecognizeResponse};
use crate::{ContextData, RecognizeData};

use actix_multipart::Multipart;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponse, Responder};

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

async fn extract_multiform_data(mut payload: Multipart) -> Result<String, anyhow::Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!(
            "/Users/breadrock/Projects/audio-to-text/upload/{}",
            filename
        );
        let filepath2 = filepath.clone();
        let create_file_result = web::block(|| File::create(filepath)).await.unwrap();
        if create_file_result.is_err() {
            let err = create_file_result.err().unwrap();
            let msg = format!("Failed while creating tmp file: {}", err);
            log::error!("{}", msg.as_str());
            return Err(anyhow::Error::msg(msg));
        }

        let mut file = create_file_result.unwrap();
        while let Some(read_chunk_result) = field.next().await {
            if read_chunk_result.is_err() {
                let err = read_chunk_result.err().unwrap();
                let msg = format!("Failed while extracting chunk: {}", err);
                log::error!("{}", msg);
                return Err(anyhow::Error::msg(msg));
            }

            let data = read_chunk_result.unwrap();
            let file_res = web::block(move || file.write_all(&data).map(|_| file))
                .await
                .unwrap();

            if file_res.is_err() {
                let err = file_res.err().unwrap();
                let msg = format!("Failed while extracting chunk: {}", err);
                log::error!("{}", msg);
                return Err(anyhow::Error::msg(msg));
            }

            file = file_res.unwrap()
        }

        return Ok(filepath2);
    }

    Err(anyhow::Error::msg(
        "Failed while extracting multiform".to_string(),
    ))
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
