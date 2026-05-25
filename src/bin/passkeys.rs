// this bin is a wasm binary. main fn here to allow it to compile

use wasm_bindgen::{JsCast, JsError, JsValue, UnwrapThrowExt, prelude::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlElement, PublicKeyCredentialRequestOptions};
// #[cfg(all(target_arch = "wasm32",target_os="unknown"))]
pub fn main() {
    #[allow(unused)]
    let _ = wasm32_browser_main();
}

pub async fn is_conditional_mediation_available() -> bool {
    // is conditional mediation available
    JsFuture::from(web_sys::PublicKeyCredential::is_conditional_mediation_available())
        .await
        .unwrap_or_default()
        .as_bool()
        .unwrap_or(false)
}

// this is the real main function
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "unknown"),
    wasm_bindgen::prelude::wasm_bindgen(start)
)]
#[allow(dead_code)]
async fn wasm32_browser_main() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    let _ = tracing_wasm::try_set_as_global_default().ok();
    tracing::info!("We hear you houston. registering app and event listeners");
    let window = web_sys::window()
        .ok_or_else(|| wasm_bindgen::JsError::new("Failed to get window reference"))?;
    let document = window.document().ok_or_else(|| {
        wasm_bindgen::JsError::new("Failed to get document reference from the window document")
    })?;

    // we are going for a 'frictionless' click to continue. stage the browsers creds using this flow
    let navigator = window.navigator();
    let credentials = navigator.credentials();

    // is conditional mediation available

    // our special sauce. opts as binary

    let raw_challenge = js_sys::Reflect::get(&window, &"WEBAUTHN_CHALLENGE_BIN".into())?;
    let challenge_buffer = js_sys::Uint8Array::new(&raw_challenge).to_vec();
    tracing::debug!("{} {challenge_buffer:?}", challenge_buffer.len());
    let challenge: webauthn_rs_proto::auth::RequestChallengeResponse =
        bitcode::decode(&challenge_buffer)
            .map_err(|e| wasm_bindgen::JsError::new(&e.to_string()))?;
    
    let challenge2 = challenge.clone();


    // we need to deeply customize the CRO because webauthn-rs does not specifiy a lot of these options



    let cro: web_sys::CredentialRequestOptions = challenge.into();
        let mut pubkey = cro.get_public_key().unwrap_throw();
        pubkey.set_hints(&["client-device".into(),"hybrid".into(),"security-key".into()]); //,"hybrid".into(),"security-key".into()
        #[allow(deprecated)]
        pubkey.set_user_verification(web_sys::UserVerificationRequirement::Preferred);
        
    cro.set_public_key(&pubkey);
    tracing::debug!("cro {cro:?}");
    // specify the 'hints' or 'what kind of ui do we prefer'
    // we prefer something like dashlane first if available, then the browser, then a security key  


    let cro2: web_sys::CredentialRequestOptions = challenge2.into();
    // js_sys::Object::assign(&pubkey2.unchecked_ref(), &pubkey.unchecked_ref());
    cro2.set_public_key(&pubkey);

    if is_conditional_mediation_available().await {
        js_sys::Reflect::set(cro.as_ref(), &"mediation".into(), &"conditional".into());
        tracing::debug!("browser supports conditional mediation. asking browser to apply it");
        if let Some(input) = document.get_element_by_id("autofill-trigger") {
            let input: web_sys::HtmlInputElement = input.dyn_into()?;
            input.focus()?; // This "wakes up" the browser's primed autofill
        }
    }


    // ask the browser to start a conditional mediation request. if it succeds, it will autofill the input field
    // for us, otherwise we need an abortcontroller to tell the browser to stop waiting
    let abort_controller= web_sys::AbortController::new()?;
    let signal = abort_controller.signal();
    // let cro2 = cro.clone();
    js_sys::Reflect::set(&cro.as_ref(),&"signal".into(),&signal.into());
    wasm_bindgen_futures::spawn_local(async move {
        // set up the abort controller so it cancels when the user clicks on the continue button.

        let creds_promise = credentials.get_with_options(&cro).unwrap();

        let js_value = match JsFuture::from(creds_promise).await {
            Err(e) => {tracing::error!("Error conditinoal mediation: {e:?}");
            return;
        }
            Ok(v)=>{v}
        };
        let credential = js_value.dyn_into::<web_sys::PublicKeyCredential>().unwrap_throw();
        let response = credential
            .response()
            .dyn_into::<web_sys::AuthenticatorAssertionResponse>().unwrap();
        tracing::debug!("assertion response: {response:?}");
    });

    let continue_btn = document.get_element_by_id("continue-btn")
    .ok_or_else(||JsError::new("continue button was not available. the user must select a passkey manually or the server must accept the input"))?;
    
    // we have to set this here because closure expects fnmut
    let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        event.prevent_default();
        // use the generated signal above to tell the abort controller to cancel
        let cro2 = cro2.clone();
        let _ = js_sys::Reflect::set(&cro2, &"mediation".into(), &"optional".into()).unwrap_throw();
        let pk = cro2.get_public_key().unwrap_throw();
        pk.set_user_verification(web_sys::UserVerificationRequirement::Required);
        js_sys::Reflect::set(&pk, &"authenticatorAttachment".into(), &"platform".into()).unwrap_throw();
        cro2.set_public_key(&pk);
        let abort_controller = abort_controller.clone();
        let navigator = navigator.clone();
        wasm_bindgen_futures::spawn_local(async move  {
            // let abort_controller = abort_controller.clone();
            abort_controller.abort_with_reason(&"cancelled:continuefired".into());
            // ask the browser again
            let promise = navigator.credentials().get_with_options(&cro2).unwrap_throw();
            let js_value = JsFuture::from(promise).await.unwrap_throw();
            let credential = js_value.dyn_into::<web_sys::PublicKeyCredential>().unwrap_throw();
            let response = credential
            .response()
            .dyn_into::<web_sys::AuthenticatorAssertionResponse>().unwrap();
            tracing::debug!("assertion response: {response:?}");
        });
    });

    let continue_btn: HtmlElement = continue_btn.dyn_into()?;
    continue_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());

    closure.forget();

    Ok(())
}
