mod native_log;
use napi_derive_ohos::napi;

#[cfg(not(target_arch = "wasm32"))]
mod client;