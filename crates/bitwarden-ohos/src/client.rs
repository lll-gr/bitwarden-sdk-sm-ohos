use crate::native_log::{hilog_global_options, init_panic_hook, init_tracing_subscriber};
use bitwarden_json::client::Client as JsonClient;
use napi_derive_ohos::napi;

#[napi]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

// 原日志系统
// fn convert_level(level: LogLevel) -> log::LevelFilter {
//     match level {
//         LogLevel::Trace => log::LevelFilter::Trace,
//         LogLevel::Debug => log::LevelFilter::Debug,
//         LogLevel::Info => log::LevelFilter::Info,
//         LogLevel::Warn => log::LevelFilter::Warn,
//         LogLevel::Error => log::LevelFilter::Error,
//     }
// }

#[napi]
pub struct BitwardenClient(JsonClient);

#[napi]
impl BitwardenClient {
    #[napi(constructor)]
    pub fn new(settings_input: Option<String>, log_level: Option<LogLevel>) -> Self {
        // Initialize panic hook for better error reporting
        init_panic_hook();

        // Set up hilog global options with default domain and tag
        hilog_global_options(0x0001, "BitwardenSDK".to_string());

        // Initialize tracing subscriber for native logging
        init_tracing_subscriber();

        // 原日志系统
        // let _ = env_logger::Builder::from_default_env()
        //     .filter_level(convert_level(log_level.unwrap_or(LogLevel::Info)))
        //     .try_init();

        Self(bitwarden_json::client::Client::new(settings_input))
    }

    #[napi]
    pub async fn run_command(&self, command_input: String) -> String {
        self.0.run_command(&command_input).await
    }
}
