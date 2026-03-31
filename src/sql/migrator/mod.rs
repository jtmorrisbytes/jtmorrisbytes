use std::sync::Arc;

// use barrel::Table;



// the bring up protocol. WTF to do about this?
// CREATE TABLE IF NOT EXISTS with empty columns
// ADD COLUMN IF NOT EXISTS with data types
// ALTER COLUMN IF EXISTS set data type
// add or check foreign key constraints
// check unique constraints
// add check constraints
// add check indexes
// final check pass?


// refinery::embed_migrations!("./migrations");

pub fn bring_up<'client>(
    client: &'client mut tokio_postgres::Client,
) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> {
    

    async move {
        // let txn = txn.await?;
        // client.con
        // migrations::runner().run_async(client).await?;
        
        Ok(())
    }
    
}

#[tokio::test]
pub async fn it_migrates() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@localhost:5432/accounting_db".to_string());
    let mut conn = crate::sql::connect::connect(&database_url).await?;
    let client = &mut conn.client;
    self::bring_up(client).await?;

    Ok(())
}
