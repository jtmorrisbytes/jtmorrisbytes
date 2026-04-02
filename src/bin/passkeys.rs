// this bin is a wasm binary. main fn here to allow it to compile

// #[cfg(all(target_arch = "wasm32",target_os="unknown"))]
pub fn main() {
    #[allow(unused)]
    let _ = wasm32_browser_main();
}


// this is the real main function
#[cfg_attr(all(target_arch = "wasm32",target_os="unknown"),wasm_bindgen::prelude::wasm_bindgen(start))]
#[allow(dead_code)]
async fn wasm32_browser_main() {
    console_error_panic_hook::set_once();
    let _ = tracing_wasm::try_set_as_global_default().ok();
    tracing::info!("We hear you houston. registering app and event listeners");
}