use rocket::{futures::Stream, http::CookieJar};

#[derive(rocket::Responder)]
pub enum RenderIndexOutcome<S>
    where S:Stream<Item = String>
{
    OkRespondWithStream(super::HtmlStream<S>),
    NotAuthorizedWithRedirect(rocket::response::Redirect)
}



#[rocket::get("/",format="text/html")]
pub fn render_html(cookie_jar: &CookieJar<'_>) -> RenderIndexOutcome<impl Stream<Item=String>> {
    let session_id = match cookie_jar.get("session_id") {
        None=> {
            return RenderIndexOutcome::NotAuthorizedWithRedirect(rocket::response::Redirect::to(rocket::uri!(crate::rockets::authentication::render_login_page)));
        }
        Some(id)=>id
    };
    let s = async_stream::stream! {
        yield crate::templates::doctype_html5();
        yield "<html><head>".to_string();
        yield crate::templates::meta_charset("utf-8");
        yield crate::templates::meta_viewport();
        yield crate::templates::title("Hello world");
        yield crate::templates::bootstrap_js_async();
        yield crate::templates::bootstrap_css_preload();
        yield "</head></body>".to_string();
        yield "TODO: MAIN CONTENT".to_string();
        yield "</body></html>".to_string();
    };
    RenderIndexOutcome::OkRespondWithStream(super::HtmlStream(s))
}