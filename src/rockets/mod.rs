pub mod authentication;
pub mod site_root;
use std::ops::DerefMut;
use rocket::{
    Build, Rocket,
    fairing::{AdHoc, Fairing, Info, Kind},
    futures::StreamExt,
    http::Header, routes,
};
// pub mod browserhash;
// #[derive(Database)]
// #[database("primary")]
pub struct PrimaryDatabase(PgPool);

#[derive(Debug)]
pub struct DbError(String);

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database Error: {}", self.0)
    }
}

impl std::error::Error for DbError {
    fn description(&self) -> &str {
        &self.0
    }
}

pub struct PgPool(deadpool_postgres::Pool);

#[rocket::async_trait]
impl rocket_db_pools::Pool for PgPool {
    type Connection = deadpool_postgres::Object;
    type Error = DbError;
    async fn init(figment: &rocket::figment::Figment) -> Result<Self, Self::Error> {
        // this didnt work right for some reason
        // let c : rocket_db_pools::Config = figment.focus("databases.primary").extract().map_err(|e|
        //     DbError(format!("Failed to configure Database from rocket::figment: {e}"))
        // )?;
        let url = std::env::var("DATABASE_URL").unwrap();

        let pool = crate::sql::connect::pooled(&url)
            .map_err(|e| DbError(format!("Failed to create Pool: {e}")))?;
        Ok(Self(pool))
    }
    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        let c = self
            .0
            .get()
            .await
            .map_err(|e| DbError(format!("Failed to get Pool Connection: {e}")))?;
        Ok(c)
    }
    async fn close(&self) {
        self.0.close();
    }
}
impl std::ops::Deref for PrimaryDatabase {
    type Target = PgPool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PrimaryDatabase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::convert::From<PgPool> for PrimaryDatabase {
    fn from(value: PgPool) -> Self {
        Self(PgPool(value.0.clone()))
    }
}

impl rocket_db_pools::Database for PrimaryDatabase {
    type Pool = PgPool;
    const NAME: &'static str = "primary";
}

struct CspFairing;

#[rocket::async_trait]
impl Fairing for CspFairing {
    fn info(&self) -> Info {
        Info {
            name: "Add CSP Header",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _request: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        response.set_header(Header::new(
            "Content-Security-Policy",
            r#"default-src 'self'; frame-ancestors 'none'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval' https://cdn.jsdelivr.net; connect-src 'self' https://cdn.jsdelivr.net; style-src 'self' https://cdn.jsdelivr.net; object-src 'none'"#
        ));
    }
}

use rocket_db_pools::{Database, rocket};

pub struct HtmlStream<S>(pub S);

impl<'r, 'o: 'r, S, I> rocket::response::Responder<'r, 'o> for HtmlStream<S>
where
    S: rocket::futures::Stream<Item = I> + Send + 'o,
    I: AsRef<str> + Send + 'o,
{
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        let byte_stream = self
            .0
            .map(|s| Ok::<_, std::io::Error>(bytes::Bytes::from(s.as_ref().to_string())));
        let reader = tokio_util::io::StreamReader::new(byte_stream);
        rocket::Response::build()
            .header(rocket::http::ContentType::HTML)
            .streamed_body(reader)
            .ok()
    }
}

// use crate::{CspFairing, PrimaryDatabase};
// #[launch]
pub fn rocket() -> Rocket<Build> {
    dotenvy::dotenv().ok();
    Rocket::build()
        .mount(
            "/",
            rocket::routes![
                crate::rockets::site_root::render_html,
                crate::rockets::authentication::render_login_page
            ],
        )
        .mount(
            "/public",
            rocket::fs::FileServer::from(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("public"),
            ),
        )
        // content security policy
        .attach(PrimaryDatabase::init())
        .attach(CspFairing)
}
