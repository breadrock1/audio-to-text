use crate::errors;
use crate::healthcheck;
use crate::whisper;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        description="There are swagger docs of audio-to-text service endpoints."
    ),
    paths(
        healthcheck::routes::check_health,
        whisper::routes::upload_form,
        whisper::routes::recognize_file,
    ),
    components(
        schemas(
            errors::ErrorResponse,
            errors::SuccessfulResponse,
            whisper::forms::RecognizeParameters,
            whisper::forms::RecognizeResponse,
        )
    ),
    tags ((
        name = "audio-to-text REST Api",
        description = "There is simple audio-to-text service based on Rust and Whisper."
    ))
)]
pub struct ApiDoc;

pub fn build_scope() -> SwaggerUi {
    let openapi = ApiDoc::openapi();
    SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", openapi.clone())
}
