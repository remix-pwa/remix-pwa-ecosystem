use wasm_bindgen::prelude::*;

pub mod client;
pub mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen]
pub fn init_console_panic() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);
}

#[wasm_bindgen(start)]
fn run() {
    init_console_panic();
    debug("@remix-pwa/client WASM module loaded successfully");
}