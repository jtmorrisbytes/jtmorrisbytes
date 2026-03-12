use sqlx::{Database, Row};

#[async_trait::async_trait]
pub trait SchemaInspector<DB: sqlx::Database> {
    async fn get_columns(&mut self,for_table_name: &str)-> Result<(),anyhow::Error>;
}
// #[async_trait::async_trait]
// impl<'c> SchemaIntrospector<sqlx::Sqlite> for sqlx::Transaction<'c,sqlx::Sqlite>{
//     fn get_tables(&mut self) {
        
//     }
// }


const PRAGMA_TABLE_INFO_SQL: &str = r#"SELECT "cid","name","type","notnull","dflt_value","pk" from pragma_table_info(?)"#;


#[async_trait::async_trait]
impl SchemaInspector<sqlx::Sqlite> for sqlx::Transaction<'_,sqlx::Sqlite> {
    async fn get_columns(&mut self,for_table_name: &str) -> Result<(),anyhow::Error> {
        // // Concrete types are safe here!
        // let rows = SqliteTableInfo::query()
        //     .fetch_all(&mut **self)
        //     .await?;
        // Ok(rows.into_iter().map(Into::into).collect())
        todo!()
    }
}

#[async_trait::async_trait]
impl<'t> SchemaInspector<sqlx::any::Any> for sqlx::Transaction<'t,sqlx::any::Any>
{
    async fn get_columns(&mut self,for_table_name: &str) -> Result<(),anyhow::Error> {
        match self.backend_name() {
            sqlx::sqlite::Sqlite::NAME => {
                let t = sqlx::query(PRAGMA_TABLE_INFO_SQL).bind(for_table_name).fetch_all(&mut **self).await?;
                for r in t {
                    let name = r.try_get("name").unwrap_or("");
                    dbg!(name);
                }
            }
            _=> {
                todo!("not implemented yet")
            }
        }
        // // Concrete types are safe here!
        // let rows = SqliteTableInfo::query()
        //     .fetch_all(&mut **self)
        //     .await?;
        // Ok(rows.into_iter().map(Into::into).collect())
        Ok(())
    }
}




// #[async_trait::async_trait]
// impl<'c,SqlxDatabase> SchemaIntrospector<SqlxDatabase> for sqlx::Transaction<'c,SqlxDatabase>
// where SqlxDatabase: sqlx::Database {
//     fn get_tables(&mut self) {
        
//     }
// }