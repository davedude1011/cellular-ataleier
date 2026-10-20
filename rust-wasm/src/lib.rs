use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

pub mod config;
pub mod core;
pub mod entities;
pub mod wasm;

#[wasm_bindgen(js_name = wasmMemory)]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen(start)]
pub fn init_console_logger() {
    console_error_panic_hook::set_once();
    console_log("wasm started!");
}

#[inline]
pub fn console_log(s: impl AsRef<str>) {
    web_sys::console::log_1(&s.as_ref().into());
}
