#[cfg(not(target_arch = "wasm32"))]
#[rocket::main]
pub async fn main(){
    jtmb::rockets::rocket().ignite().await.unwrap().launch().await.unwrap();
}

#[cfg(all(target_arch = "wasm32",target_os="unknown"))]
pub fn main() {
    let _ = wasm32_browser_main();
}
#[cfg_attr(all(target_arch = "wasm32",target_os="unknown"),wasm_bindgen::prelude::wasm_bindgen(start))]
async fn wasm32_browser_main() {
    tracing_wasm::set_as_global_default();
    tracing::info!("We hear you houston. registering app and event listeners");
}