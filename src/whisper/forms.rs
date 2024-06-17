use utoipa::{IntoParams, ToSchema};

#[derive(serde::Deserialize, IntoParams, ToSchema)]
pub(crate) struct RecognizeParameters {
    language: Option<String>,
    use_threads: i32,
    resample_rate: u32,
    enable_translate: bool,
    enable_print_special: bool,
    enable_print_progress: bool,
    enable_print_realtime: bool,
    enable_print_timestamps: bool,
}

#[allow(dead_code)]
impl RecognizeParameters {
    pub fn get_lang(&self) -> Option<&str> {
        match &self.language {
            None => None,
            Some(lang) => Some(lang.as_str())
        }
    }
    pub fn get_resample_rate(&self) -> u32 {
        self.resample_rate
    }
    pub fn get_threads(&self) -> i32 {
        self.use_threads
    }
    pub fn is_translate_enable(&self) -> bool {
        self.enable_translate
    }
    pub fn is_print_spec_enable(&self) -> bool {
        self.enable_print_special
    }
    pub fn is_print_progress_enable(&self) -> bool {
        self.enable_print_progress
    }
    pub fn is_print_realtime_enable(&self) -> bool {
        self.enable_print_realtime
    }
    pub fn is_print_timestamp_enable(&self) -> bool {
        self.enable_print_timestamps
    }
}

impl Default for RecognizeParameters {
    fn default() -> Self {
        RecognizeParameters {
            language: Some("en".to_string()),
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
    pub frame_id: i32,
    pub frame_start: i64,
    pub frame_end: i64,
    pub text: String,
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
