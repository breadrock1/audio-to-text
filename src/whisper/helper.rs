use actix_multipart::Multipart;
use actix_web::web;
use futures_util::{StreamExt, TryStreamExt};
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Write;

pub(crate) async fn extract_multiform_data(
    mut payload: Multipart,
) -> Result<String, anyhow::Error> {
    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(|err| extract_error(err, "Failed while processing stream"))?
    {
        let content_type = field.content_disposition();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./upload/{}", filename);
        let filepath_cln = filepath.clone();

        let create_file_result = web::block(|| File::create(filepath)).await.unwrap();
        if create_file_result.is_err() {
            let err = create_file_result.err().unwrap();
            return Err(extract_error(err, "creating tmp file"));
        }

        let mut file = create_file_result.unwrap();
        while let Some(read_chunk_result) = field.next().await {
            if read_chunk_result.is_err() {
                let err = read_chunk_result.err().unwrap();
                return Err(extract_error(err, "extracting chunk"));
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

        return Ok(filepath_cln);
    }

    let msg = "Failed while extracting multiform".to_string();
    Err(anyhow::Error::msg(msg))
}

fn extract_error<T: Debug + Display>(err: T, msg: &str) -> anyhow::Error {
    let msg = format!("Failed while {}: {}", msg, err);
    log::error!("{}", msg);
    return anyhow::Error::msg(msg);
}
