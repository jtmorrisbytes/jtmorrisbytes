async function start() {
    let r = await wasm_bindgen("/public/static/wasm/passkeys/passkeys_bg.wasm");
    // I dont think we have to do this
    // r.wasm32_browser_main();
}
start();
