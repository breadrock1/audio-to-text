use crate::endpoints::hello::*;
use crate::endpoints::recognizer::*;
use crate::errors::{ErrorResponse, SuccessfulResponse};
use crate::transformer::client::{RecognizeParameters, RecognizeResponse};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        description="There are swagger docs of audio-to-text service endpoints."
    ),
    paths(
        hello,
        upload_file_form,
        recognize_audio,
        recognize_audio_full_text,
    ),
    components(
        schemas(
            ErrorResponse,
            SuccessfulResponse,
            RecognizeParameters,
            RecognizeResponse,
        )
    ),
    tags ((
        name = "audio-to-text REST Api",
        description = "There is simple audio-to-text service based on Rust and Whisper."
    ))
)]
pub struct ApiDoc;
