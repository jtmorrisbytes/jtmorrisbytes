use futures::Stream;
pub fn login_box_fragment(challenge_bytes: &[u8]) -> impl Stream<Item=String> {
    // if this fails no bytes will be serialized
    let mut iter = challenge_bytes.iter();
    async_stream::stream! {
        yield "<div>".to_string();
        // generate a script tag that contains the challenge
        yield "<script>window.WEBAUTHN_CHALLENGE_BIN=new Uint8Array([".to_string();
        // yield the first, prevents needing to check for commas
        while let Some(byte) = iter.next() {
            yield format!("{},",byte.to_string());
        }
        yield "]);</script>".to_string();
        // warning. this also assumes that you will load the wasm on first load
        yield r#"<script src='/public/static/wasm/passkeys/passkeys.js'></script>"#.to_string();
        yield r#"<script src='/public/static/js/init_login_box_fragment.js'></script>"#.to_string();
        yield r#"<h1>Welcome</h1><p>\
        Click the button to continue, when prompted please allow the browser to create a passkey for our website\
        on your behalf</p>
        <!-- this is required to make conditional mediation work -->
        <form>
        <input type="text" 
       name="username" 
       autocomplete="username webauthn" 
       id="autofill-trigger" 
       style="display: block; opacity: 0; position: absolute; z-index: -1;">
        <button id='continue-btn'>Please Click here to continue</button></form></div>"#.to_string()
    }
}


