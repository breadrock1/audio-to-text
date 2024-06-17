use std::str::FromStr;

pub struct WhisperClientConfig {
    model_path: String,
    enable_gpu: bool,
}

impl WhisperClientConfig {
    pub fn from_env() -> Self {
        let model_path = std::env::var("WHISPER_MODEL_PATH")
            .expect("failed while getting WHISPER_MODEL_PATH value");
        let enable_gpu_data = std::env::var("WHISPER_ENABLE_GPU")
            .expect("failed while getting WHISPER_ENABLE_GPU value");
        let enable_gpu = bool::from_str(enable_gpu_data.as_str())
            .expect("incorrect WHISPER_ENABLE_GPU value");

        WhisperClientConfig {
            model_path,
            enable_gpu,
        }
    }
    pub fn get_model_path(&self) -> &str {
        self.model_path.as_str()
    }
    pub fn is_gpu_enabled(&self) -> bool {
        self.enable_gpu
    }
}

impl Default for WhisperClientConfig {
    fn default() -> Self {
        WhisperClientConfig {
            model_path: "./models/ggml-base.en.bin".to_string(),
            enable_gpu: false,
        }
    }
}
