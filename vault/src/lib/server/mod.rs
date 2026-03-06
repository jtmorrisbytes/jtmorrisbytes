// we plan on implementing a vault ui based on https and html
// reexport the rocket crate;
pub use rocket;
use rocket::{Build, Rocket};



// bitches we crazy yo!
// we goin to compile a rust crate as a ui library at dev time so we can SSR render that webasembly yo

// obviously only use this during developmentt


/// displays hello world to the user
#[rocket::get("/")]
pub fn get_index() -> &'static str  {
    // wasmtime::
    "HELLO FROM VAULT"
}


// WE ARE HTML AGAIN LOL.
// creates a custom rocket instance based on the vaults config

pub fn build_rocket(vault_config: crate::config::Config) -> Rocket<Build> {
    let config = rocket::Config::figment()
    .merge(("address","127.0.0.1"))
    .merge(("port",8765_u16))
    .merge(("shutdown.ctrlc",false));

    rocket::custom(config).mount("/", rocket::routes![get_index])
    .mount("/public/static", rocket::fs::FileServer::from(vault_config.vault_data_directory))
}

