pub mod sql;
use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;

use postgres_types::{FromSql, ToSql};
// use rustls::crypto::hash::Output;
use tokio::io::AsyncWriteExt;
// use crate::core::connect;
use tokio_postgres::Transaction;

// use crate::jtmb::sql;
use crate::sql::connect::SingleConnHandle;



pub fn c() -> impl Future<Output = Result<SingleConnHandle, Box<dyn std::error::Error>>> + Send {
    async move {
        let url = std::env::var("DATABASE_URL")?;
        let conn = crate::sql::connect::connect(&url).await?;
        // run any migration code that may need to be performed
        Ok(conn)
    }
}

pub fn bring_up_db<'client>(
    c: &'client mut tokio_postgres::Client,
) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + use<'client> {
    async move {
        // let mut c = c().await?;
        let migs = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR")?)
            .join("migrations")
            .canonicalize()?;
        let mut rd = tokio::fs::read_dir(migs).await?;
        let mut f = vec![];
        while let Some(r) = rd.next_entry().await? {
            // let ft = r.metadata().await?;
            if r.path().extension().unwrap().to_str().unwrap() == "sql" {
                f.push(r);
            }
        }
        f.sort_by_key(|k| k.file_name());

        let t = c
            .build_transaction()
            .deferrable(true)
            .isolation_level(tokio_postgres::IsolationLevel::Serializable)
            .read_only(false)
            .start()
            .await?;
        for e in f {
            let f = tokio::fs::read_to_string(e.path()).await?;
            // let p = t.prepare(&f).await?;
            // let a: &[&(dyn ToSql + Send + Sized)] = &[];
            t.batch_execute(&f).await?;
            // t.execute_raw::<_, _, Statement>(&p, std::iter::empty::<&(dyn ToSql + Sync)>()).await?;
        }
        t.commit().await?;

        // crate::migrator::bring_up(&mut c.client).await;
        Ok(())
    }
}

pub fn workspace_root() -> Result<String, Box<dyn std::error::Error>> {
    std::env::var("CARGO_WORKSPACE_DIR")
    .or_else(|_| std::env::var("CARGO_MANIFEST_DIR")).map_err(|_| "Failed to detect workspace root. either CARGO_WORKSPACE_DIR or CARGO_MANIFEST_DIR must be set".into())
}
