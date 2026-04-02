use rocket::{futures::Stream, http::{Cookie, CookieJar}};
use crate::templates::*;
#[rocket::get("/authentication/start")]
pub async fn render_login_page(cookie_jar: &CookieJar<'_>) -> crate::rockets::HtmlStream<impl Stream<Item=String>> {
    let webauthn = crate::passkeys::build_webauthn_instance();
    let (challenge,auth) = webauthn.start_passkey_authentication(&[]).unwrap();
    
    
    let c = Cookie::new("PasskeyAuthState", crate::passkeys::serialize_passkey_auth_state_as_hex_encoded_binary(&auth));
    cookie_jar.add_private(c);

    let s = rocket::async_stream::stream! {
        yield r#"
        <html>
        <head>"#.to_string();
        yield title("Please Log In");
        yield meta_viewport();
        yield meta_charset("utf-8");
        yield bootstrap_js_async();
        yield bootstrap_css_preload();
        r#"</head><body>"#;
        
        for await txt in crate::templates::authentication::login_box_fragment(challenge) {
            yield txt.to_string();
        }
        yield r#"</body></html>"#.to_string();
    };

    crate::rockets::HtmlStream(s)
}