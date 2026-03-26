// use crate::core::connect;
use tokio_postgres::Transaction;

use crate::core::connect::SingleConnHandle;

pub fn c() -> impl Future<Output = Result<SingleConnHandle, Box<dyn std::error::Error>>> + Send {
    async move {
        let url = std::env::var("DATABASE_URL")?;
        let conn = crate::core::connect::connect(&url).await?;
        // run any migration code that may need to be performed
        Ok(conn)
    }
}

pub fn bring_up_db() -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send {
    async move {
        let mut c = c().await?;
        crate::migrator::bring_up(&mut c.client).await;
        Ok(())
    }
}
