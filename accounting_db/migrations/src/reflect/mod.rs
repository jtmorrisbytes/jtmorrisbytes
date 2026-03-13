pub mod sqlite;
pub mod any;

use sqlx::{Database, Row, prelude::FromRow};



#[derive(Debug)]
pub enum AnyColumnInfo {
    Sqlite(sqlite::SqliteColumnInfo),
}

#[async_trait::async_trait]
pub trait SchemaInspector<DB: sqlx::Database> {
    type ColumnInfo: Send + Sync;
    type TableInfo: Send + Sync;
    async fn get_columns(
        &mut self,
        for_table_name: &str,
    ) -> Result<Vec<Self::ColumnInfo>, anyhow::Error>;
}
