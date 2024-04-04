use tokio::process::Command;

pub(crate) async fn resample_audio_file(
    file_path: &str,
    resample_rate: u32,
) -> Result<String, std::io::Error> {
    let uuid_value = uuid::Uuid::new_v4();
    let output_file = format!("/tmp/{}.wav", uuid_value);
    let exec_result = Command::new("ffmpeg")
        .arg("-i")
        .arg(file_path)
        .arg("-ac")
        .arg("1")
        .arg("-ar")
        .arg(resample_rate.to_string().as_str())
        .arg(output_file.as_str())
        .output()
        .await;

    match exec_result {
        Ok(_) => Ok(output_file),
        Err(err) => Err(err),
    }
}
