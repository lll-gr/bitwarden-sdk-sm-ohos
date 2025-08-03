mod native_log;

// 这个意思是只在非 wasm32 平台下编译
#[cfg(not(target_arch = "wasm32"))]
mod client;