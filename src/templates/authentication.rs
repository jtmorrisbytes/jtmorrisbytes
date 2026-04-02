use futures::Stream;
pub fn login_box_fragment(challenge: webauthn_rs::prelude::RequestChallengeResponse) -> impl Stream<Item=String> {
    // if this fails no bytes will be serialized
    let b = bitcode::encode(&challenge);
    let mut iter = b.into_iter();
    async_stream::stream! {
        yield "<div>".to_string();
        // generate a script tag that contains the challenge
        yield "<script>const WEBAUTHN_CHALLENGE_BIN=new Uint8Array([".to_string();
        // yield the first, prevents needing to check for commas
        if let Some(byte) = iter.next() {
            yield byte.to_string();
        }
        while let Some(byte) = iter.next() {
            yield format!("{},",byte.to_string());
        }
        yield "]);</script>".to_string();

        yield r#"<script src='/public/static/wasm/passkeys/passkeys.js'></script>"#.to_string();
        yield r#"<script src='/public/static/js/init_login_box_fragment.js'></script>"#.to_string();
        yield "<p>loginbox</p></div>".to_string()
    }
}


