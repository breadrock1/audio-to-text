use crate::whisper::config::WhisperClientConfig;
use crate::whisper::forms::{RecognizeParameters, RecognizeResponse};
use crate::whisper::resampler::resample_audio;

use std::ffi::c_int;
use std::io::BufReader;
use hound::{WavReader, WavSpec};
use tokio::io;

use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters, WhisperError, WhisperState};

type RecognizeResult<T> = Result<T, WhisperError>;

pub struct WhisperClient {
    client: WhisperContext,
}

impl WhisperClient {
    pub fn new(cfg: &WhisperClientConfig) -> Self {
        let mut cxt_params = WhisperContextParameters::default();
        cxt_params.use_gpu(cfg.is_gpu_enabled());

        let whisper_ctx = WhisperContext::new_with_params(cfg.get_model_path(), cxt_params)
            .expect("Failed while loading whisper model...");

        WhisperClient {
            client: whisper_ctx,
        }
    }

    pub(crate) fn recognize_chunk(&self, audio_data: &[u8], params: &RecognizeParameters) -> RecognizeResult<Vec<RecognizeResponse>> {
        let strategy = SamplingStrategy::Greedy { best_of: 1 };
        let mut full_params = FullParams::new(strategy);
        full_params.set_language(params.get_lang());
        full_params.set_n_threads(params.get_threads());
        full_params.set_translate(params.is_translate_enable());
        full_params.set_print_special(params.is_print_spec_enable());
        full_params.set_print_progress(params.is_print_progress_enable());
        full_params.set_print_realtime(params.is_print_realtime_enable());
        full_params.set_print_timestamps(params.is_print_timestamp_enable());

        let buffered = BufReader::new(audio_data);
        let mut reader = WavReader::new(buffered)
            .map_err(|err| {
                log::error!("Failed while reading resampled audio file: {}", err);
                WhisperError::FailedToDecode
            })?;

        let recognize_res = self.recognize(&mut reader, full_params);
        recognize_res
    }

    pub(crate) async fn recognize_file(&self, file_path: &str, params: &RecognizeParameters) -> RecognizeResult<Vec<RecognizeResponse>> {
        let audio_file_path = resample_audio(file_path, params.get_resample_rate())
            .await
            .map_err(|err| {
                log::error!("Failed while resampling audio file: {}", err);
                WhisperError::DecodeNotComplete
            })?;

        let mut reader = WavReader::open(audio_file_path.as_str())
            .map_err(|err| {
                log::error!("Failed while reading resampled audio file: {}", err);
                WhisperError::FailedToDecode
            })?;

        let strategy = SamplingStrategy::Greedy { best_of: 1 };
        let mut full_params = FullParams::new(strategy);
        full_params.set_language(params.get_lang());
        full_params.set_n_threads(params.get_threads());
        full_params.set_translate(params.is_translate_enable());
        full_params.set_print_special(params.is_print_spec_enable());
        full_params.set_print_progress(params.is_print_progress_enable());
        full_params.set_print_realtime(params.is_print_realtime_enable());
        full_params.set_print_timestamps(params.is_print_timestamp_enable());

        let recognize_res = self.recognize(&mut reader, full_params);
        let removing_result = Self::remove_tmp_files(audio_file_path.as_str()).await;
        if removing_result.is_err() {
            let err = removing_result.err().unwrap();
            log::warn!(
                "Failed while removing temporary file {}: {}",
                audio_file_path,
                err
            );
        }

        recognize_res
    }

    fn recognize<T>(
        &self,
        reader: &mut WavReader<BufReader<T>>,
        full_params: FullParams,
    ) -> RecognizeResult<Vec<RecognizeResponse>>
    where
        T: std::io::Read
    {
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

        let mut state = self.client.create_state()?;
        state.full(full_params, &audio[..])?;

        let num_segments = state.full_n_segments()?;
        let collected_results = (0..num_segments)
            .into_iter()
            .filter_map(|id| Self::extract_segment(&state, id).ok())
            .collect::<Vec<RecognizeResponse>>();

        Ok(collected_results)
    }

    fn extract_segment(state: &WhisperState, segment_id: c_int) -> RecognizeResult<RecognizeResponse> {
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
