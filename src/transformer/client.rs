use crate::transformer::resampler;

use hound::{WavReader, WavSpec};
use std::ffi::c_int;
use std::sync::Arc;
use tokio::io;
use tokio::sync::RwLock;
use utoipa::{IntoParams, ToSchema};
use whisper_rs::{FullParams, SamplingStrategy};
use whisper_rs::{WhisperContext, WhisperContextParameters, WhisperError, WhisperState};

#[derive(Clone)]
pub struct WhisperClient {
    client: Arc<RwLock<WhisperContext>>,
}

impl WhisperClient {
    pub fn new(model_path: &str, enable_gpu: bool) -> Self {
        let mut cxt_params = WhisperContextParameters::default();
        cxt_params.use_gpu(enable_gpu);

        let whisper_ctx = WhisperContext::new_with_params(model_path, cxt_params)
            .expect("Failed while loading whisper model...");

        WhisperClient {
            client: Arc::new(RwLock::new(whisper_ctx)),
        }
    }

    pub(crate) async fn recognize<'a>(
        &self,
        audio_file: &'a str,
        params: RecognizeParameters<'a>,
    ) -> Result<Vec<RecognizeResponse>, WhisperError> {
        let strategy = SamplingStrategy::Greedy { best_of: 1 };
        let mut full_params = FullParams::new(strategy);

        full_params.set_n_threads(params.use_threads);
        full_params.set_translate(params.enable_translate);
        full_params.set_language(params.language);
        full_params.set_print_special(params.enable_print_special);
        full_params.set_print_progress(params.enable_print_progress);
        full_params.set_print_realtime(params.enable_print_realtime);
        full_params.set_print_timestamps(params.enable_print_timestamps);

        let audio_file_path = resampler::resample_audio_file(audio_file, params.resample_rate)
            .await
            .map_err(|err| {
                log::error!("Failed while resampling audio file: {}", err);
                WhisperError::DecodeNotComplete
            })?;

        let mut reader = WavReader::open(audio_file_path).map_err(|err| {
            log::error!("Failed while reading resampled audio file: {}", err);
            WhisperError::FailedToDecode
        })?;

        #[allow(unused_variables)]
        let WavSpec {
            sample_rate,
            channels,
            bits_per_sample,
            ..
        } = reader.spec();

        if channels > 2 {
            panic!(">2 channels unsupported");
        }

        let sampled_reader = &reader
            .samples::<i16>()
            .map(|s| s.expect("invalid sample"))
            .collect::<Vec<_>>();

        let mut audio = whisper_rs::convert_integer_to_float_audio(sampled_reader);
        if channels == 2 {
            audio = whisper_rs::convert_stereo_to_mono_audio(&audio).map_err(|err| {
                log::error!("Failed while converting stereo to mono: {}", err);
                WhisperError::DecodeNotComplete
            })?;
        }

        let client = self.client.write().await;
        let mut state = client.create_state()?;
        state.full(full_params, &audio[..])?;

        let num_segments = state.full_n_segments()?;
        let collected_results = (0..num_segments)
            .into_iter()
            .filter_map(|id| Self::extract_segment(&state, id).ok())
            .collect::<Vec<RecognizeResponse>>();

        let removing_result = Self::remove_tmp_files(audio_file).await;
        if removing_result.is_err() {
            let err = removing_result.err().unwrap();
            log::warn!(
                "Failed while removing temporary file {}: {}",
                audio_file,
                err
            );
        }

        Ok(collected_results)
    }

    fn extract_segment(
        state: &WhisperState,
        segment_id: c_int,
    ) -> Result<RecognizeResponse, WhisperError> {
        let start_timestamp = state.full_get_segment_t0(segment_id)?;
        let end_timestamp = state.full_get_segment_t1(segment_id)?;
        let segment = state.full_get_segment_text(segment_id)?;
        Ok(RecognizeResponse {
            frame_id: segment_id,
            frame_start: start_timestamp,
            frame_end: end_timestamp,
            text: segment,
        })
    }

    async fn remove_tmp_files(file_path: &str) -> Result<bool, io::Error> {
        let path_to_remove = std::path::Path::new(file_path);
        match tokio::fs::remove_file(path_to_remove).await {
            Ok(_) => Ok(true),
            Err(err) => Err(err),
        }
    }
}

impl Default for WhisperClient {
    fn default() -> Self {
        #[cfg(feature = "enable-dotenv")]
        let _ = dotenv::dotenv();
        let model_path = std::env::var("WHISPER_MODEL_PATH")
            .expect("Failed while getting whisper model file path...");

        WhisperClient::new(model_path.as_str(), false)
    }
}

#[derive(serde::Deserialize, IntoParams, ToSchema)]
pub(crate) struct RecognizeParameters<'a> {
    pub language: Option<&'a str>,
    pub use_threads: i32,
    pub resample_rate: u32,
    pub enable_translate: bool,
    pub enable_print_special: bool,
    pub enable_print_progress: bool,
    pub enable_print_realtime: bool,
    pub enable_print_timestamps: bool,
}

impl Default for RecognizeParameters<'_> {
    fn default() -> Self {
        RecognizeParameters {
            language: Some("en"),
            use_threads: 1,
            resample_rate: 29000,
            enable_translate: false,
            enable_print_special: false,
            enable_print_progress: false,
            enable_print_realtime: false,
            enable_print_timestamps: false,
        }
    }
}

#[derive(Default, serde::Serialize, ToSchema)]
pub(crate) struct RecognizeResponse {
    frame_id: i32,
    frame_start: i64,
    frame_end: i64,
    text: String,
}

impl From<Vec<RecognizeResponse>> for RecognizeResponse {
    fn from(value: Vec<RecognizeResponse>) -> Self {
        let mut common_response = RecognizeResponse::default();
        let common_text = value
            .into_iter()
            .map(|rec| rec.text.to_owned())
            .collect::<Vec<String>>()
            .join(" ");

        common_response.text = common_text;
        common_response
    }
}
