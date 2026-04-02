pub mod authentication;

pub fn title(title: &str) -> String {
    format!("<title>{title}</title>")
}
pub fn doctype_html5() -> String {
    format!("<!DOCTYPE html")
}
pub fn bootstrap_css_preload() -> String {
    r#"<link rel="preload" href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css" as="style" integrity="sha384-rbsA2VBKQhggwzxH7pPCaAqO46MgnOM80zW1RWuH61DGLwZJEdK2Kadq2F9CUG65" crossorigin="anonymous" onload="this.onload=null;this.rel='stylesheet'">"#.to_string()
}
pub fn bootstrap_js_async() -> String {
   r#"<script async src="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/js/bootstrap.bundle.min.js" integrity="sha384-kenU1KFdBIe4zVF0s0G1M5b4hcpxyD9F7jL+jjXkk+Q2h455rYXK/7HAuoJl+0I4" crossorigin="anonymous"></script>"#.to_string()
}
pub fn meta_charset(charset: &str) -> String {
    format!(r#"<meta charset="{charset}">"#)
}
pub fn meta_viewport() -> String {
    format!(r#"<meta name='viewport' content='width=device-width, initial-scale=1'>"#)
}
