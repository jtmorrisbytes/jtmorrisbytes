use postgres_types::ToSql;
use tokio_postgres::Statement;

#[path="src/build/mod.rs"]
mod build;

#[path="src/sql/mod.rs"]
mod sql;
#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    dotenvy::dotenv().ok();

    // database prebuild tasks
    // println!("cargo:rerun-if-changed=src/sql");
    println!("cargo:rerun-if-changed=build.rs");
    // println!("cargo:rerun-if-changed=src/build");

    println!("cargo:rerun-if-changed=migrations");
    let mut c = self::build::c().await?;
    self::build::bring_up_db(&mut c.client).await?;
    self::build::sql::db_generate(&mut c.client).await?;

    Ok(())
}