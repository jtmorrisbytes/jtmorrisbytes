use std::collections::HashMap;

use sqlx::{Database, Execute, Executor, Row, Transaction, prelude::FromRow};

use crate::SchemaInspector;
/// Unlike other tools. this DOES NOT FILTER OUT INFORMATION SCHEMA
pub const SQL_GET_ALL_TABLE_METADATA: &str =
    r#"SELECT "name","type",* FROM sqlite_master WHERE type='table'"#;

#[derive(FromRow, Debug)]
pub struct SqliteColumnInfo {
    pub cid: i32,
    pub name: String,
    pub r#type: String,
    pub notnull: i32,
    pub dflt_value: Option<String>,
    pub pk: Option<i32>,
}

#[macro_export]
macro_rules! query_get_all_table_metadata {
    () => {
        sqlx::query($crate::drivers::sqlite::SQL_GET_ALL_TABLE_METADATA)
    };
}

pub const PRAGMA_TABLE_INFO_SQL: &str = r#"SELECT "cid","name","type","notnull","dflt_value","pk" from pragma_table_info(?) ORDER BY "cid" ASC"#;

#[macro_export]
macro_rules! meta_query_sqlite_pragma_table_info {
    ($table_name:ident) => {
        sqlx::query_as::<_, $crate::drivers::sqlite::SqliteColumnInfo>(
            $crate::drivers::sqlite::PRAGMA_TABLE_INFO_SQL,
        )
        .bind($table_name)
    };
}
#[derive(Debug)]
pub struct ColumnMetatada {
    pub table_name: String,
    pub column_id: i32,
    pub name:String,
    pub r#type:String,
    pub nullable:bool,
    pub default_value: Option<String>,
    pub is_pk: bool,
}
impl ColumnMetatada {
    pub fn from_column_info(value: &SqliteColumnInfo,tbl_name: &str) -> Self {
        Self {
            table_name: tbl_name.to_string(),
            column_id: value.cid,
            name: value.name.to_owned(),
            r#type:value.r#type.to_owned(),
            nullable: value.notnull == 0,
            default_value:value.dflt_value.to_owned(),
            is_pk: value.pk.unwrap_or(0) !=0
        }
    }
}


// impl std::convert::From<SqliteColumnInfo> for ColumnMetatada {
//     fn from(value: SqliteColumnInfo) -> Self {
//         Self {
//             column_id: value.cid,
//             name: value.name,
//             r#type:value.r#type,
//             nullable: value.notnull == 0,
//             default_value:value.dflt_value,
//             is_pk: value.pk.unwrap_or(0) !=0
//         }
//     }
// }


#[derive(Debug)]
pub struct TableMetadata {
    name:String,
    // columns:HashMap<String,ColumnMetatada>
}


#[derive(sqlx::FromRow,Debug)]
pub struct SqliteMetadataRow {
    pub r#type:String,
    pub name: String,
    pub tbl_name:String,
    pub rootpage:i32,
    pub sql:Option<String>,
}

pub enum Logic {
    And,
    Or
}
impl std::fmt::Display  for Logic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And => write!(f,"AND"),
            Self::Or => write!(f,"OR")
        }
       
    }
}


pub async fn meta_fn_sqlite_txn_get_sqlite_master<S: AsRef<str>, DatabaseDriver>(
    transaction: &mut Transaction<'_, DatabaseDriver>,
    r#type: Option<S>,
    // r#type_logic: Logic,
    name: Option<(Logic,S)>,
    // name_logic: Logic,
    tbl_name: Option<(Logic,S)>,
    rootpage: Option<(Logic,i32)>,
    // sql: Option<S>,
)-> Result<Vec<SqliteMetadataRow>,anyhow::Error> where
    DatabaseDriver: sqlx::database::Database,
    for<'ex> &'ex mut <DatabaseDriver as sqlx::Database>::Connection:
        Executor<'ex, Database = DatabaseDriver>,
    for<'qr> <DatabaseDriver as sqlx::Database>::Arguments<'qr>:
        sqlx::IntoArguments<'qr, DatabaseDriver>,
    String: sqlx::Type<DatabaseDriver>,
    for <'string> String: sqlx::Decode<'string,DatabaseDriver>,
    for <'s> String: sqlx::Encode<'s,DatabaseDriver>,
    for <'n> i32: sqlx::Encode<'n,DatabaseDriver>,
    for <'n> i32: sqlx::Decode<'n,DatabaseDriver>,
    

    i32: sqlx::Type<DatabaseDriver>,
    for <'col_index> &'col_index str: sqlx::ColumnIndex<<DatabaseDriver as sqlx::Database>::Row>

{
    use sqlx::Execute;
    let mut sql =
        r#"SELECT "type","name","tbl_name","rootpage","sql" from sqlite_master"#.to_string();
    // let mut sql: sqlx::QueryBuilder<DatabaseDriver> = sqlx::QueryBuilder::<DatabaseDriver>::new(sql);
    // let mut s: &mut sqlx::QueryBuilder<'_, DatabaseDriver>= &mut sql;
    sql.push_str(" WHERE ");
    
    let t = r#type.map(|s|s.as_ref().to_string()).unwrap_or_default();
        sql.push_str(&format!("type like '%{t}%'"));
    
    if let Some((l,name)) = name {
        let name = name.as_ref().to_string();
        sql.push_str(&format!(" {l} name like '%{name}%'"));
    }
    if let Some((l,rootpage)) = rootpage {
        sql.push_str(&format!(" {l} rootpage = {rootpage} "));
    }
    let s:Vec<SqliteMetadataRow>  = sqlx::query_as(&sql).fetch_all(&mut **transaction).await?;
    // table information
    let mut m = HashMap::new();
    let mut columns = Vec::new();
    for row in s {
        // let n = row.tbl_name.as_str();
        let cols = meta_fn_sqlite_txn_get_pragma_table_info_for_table(&mut *transaction, row.tbl_name.clone()).await?;
        for c in cols {
            columns.push(ColumnMetatada::from_column_info(&c,&row.tbl_name));
        }
        m.insert(row.tbl_name, v)
        
        
        // dbg!(cols);
    }

    todo!()
    // sqlx::query()
}



// Checks db if table T exists
/// Asks database if Table exists with Column and datatype. Only valid for sqlx::Sqlite and sqlx::Any. using any other database will result in wrong binds or syntax 
pub(crate) async fn meta_fn_does_table_exist_with_column_and_datatype<Executor, Database>(executor: Executor,table_name: String, r#type: String ) -> Result<i32,anyhow::Error>
    where
    // database bounds
     Database: sqlx::Database,
    //executor bounds
    for <'executor> Executor: sqlx::Executor<'executor,Database = Database>,
    // arguments
    for <'arguments> <Database as sqlx::Database>::Arguments<'arguments>: sqlx::IntoArguments<'arguments,Database>,
    //  type bounds
    for <'encode> String: sqlx::Encode<'encode,Database> + sqlx::Type<Database>,

    // from row
    for <'row> (i32,): sqlx::FromRow<'row,<Database as sqlx::Database>::Row>,
    for <'row> i32: sqlx::FromRow<'row,<Database as sqlx::Database>::Row>,
{
    sqlx::query_as(r#"SELECT EXISTS (SELECT 1 from pragma_table_info(?) where name = '?' and type like ?)"#)
    .bind(table_name)
    .bind(r#type)
    .fetch_one(executor)
    .await
    .map_err(|e|e.into())
}
/// Asks database if Table exists with Column and datatype. Only valid for sqlx::Sqlite using any other database will result in wrong binds or syntax
pub async fn sqlite_meta_fn_does_table_exist_with_column_and_datatype<Executor>(executor: Executor,table_name: String,r#type: String) -> Result<bool,anyhow::Error>
    where 
    for<'executor> Executor: sqlx::SqliteExecutor<'executor,Database = sqlx::Sqlite>,
    for<'row> (i32,): sqlx::FromRow<'row,<sqlx::Sqlite as sqlx::Database>::Row>,
    for<'row> i32: sqlx::FromRow<'row,<sqlx::Sqlite as sqlx::Database>::Row>


{
    meta_fn_does_table_exist_with_column_and_datatype::<Executor,sqlx::Sqlite>(executor, table_name, r#type).await.map(|t| t!=0)
}




/// asks the database for its own schema for 'pragma_table_info' returns Vec<SqliteColumnInfo>
pub async fn meta_fn_sqlite_txn_get_pragma_table_info_schema<DatabaseDriver>(
    tx: &mut sqlx::Transaction<'_, DatabaseDriver>,
) -> Result<Vec<SqliteColumnInfo>, anyhow::Error>
where
    DatabaseDriver: sqlx::Database,
    for<'ex> &'ex mut <DatabaseDriver as sqlx::Database>::Connection:
        Executor<'ex, Database = DatabaseDriver> + Send,
    for<'q> <DatabaseDriver as sqlx::Database>::Arguments<'q>:
        sqlx::IntoArguments<'q, DatabaseDriver>,
    for<'r> SqliteColumnInfo: FromRow<'r, <DatabaseDriver as sqlx::Database>::Row>,

    for<'str> &'str str: sqlx::Type<DatabaseDriver>,
    for<'en> &'en str: sqlx::Encode<'en, DatabaseDriver>,
    for<'s> &'s String: sqlx::Type<DatabaseDriver>,
    for<'s> String: sqlx::Encode<'s,DatabaseDriver>,
{
    const TABLE: &str = "pragma_table_info";
    let q = meta_query_sqlite_pragma_table_info!(TABLE);
    let t = q.fetch_all(&mut **tx).await?;
    Ok(t)
}
pub async fn meta_fn_sqlite_txn_get_pragma_table_info_for_table<'is,DatabaseDriver>(
    tx: &mut sqlx::Transaction<'_, DatabaseDriver>,
    for_table_name: String,
) -> Result<Vec<SqliteColumnInfo>, anyhow::Error>
where
    DatabaseDriver: sqlx::Database,
    for<'e> &'e mut <DatabaseDriver as sqlx::Database>::Connection:
        Executor<'e, Database = DatabaseDriver>,
    for<'q> <DatabaseDriver as sqlx::Database>::Arguments<'q>:
        sqlx::IntoArguments<'q, DatabaseDriver>,
    for<'r> SqliteColumnInfo: FromRow<'r, <DatabaseDriver as sqlx::Database>::Row>,
    for<'s> String: sqlx::Type<DatabaseDriver> + sqlx::Encode<'s,DatabaseDriver>,

    
{
    // let q = meta_query_sqlite_pragma_table_info!(for_table_name);
    
    let q = sqlx::query_as(PRAGMA_TABLE_INFO_SQL).bind(for_table_name);
    let t: Vec<SqliteColumnInfo> = q.fetch_all(&mut **tx).await?;
    dbg!(t);
    todo!()
}

#[async_trait::async_trait]
impl crate::SchemaInspector<sqlx::Sqlite> for sqlx::Transaction<'_, sqlx::Sqlite> {
    type ColumnInfo = SqliteColumnInfo;
    type TableInfo = ();
    async fn get_columns(
        &mut self,
        for_table_name: &str,
    ) -> Result<Vec<Self::ColumnInfo>, anyhow::Error> {
        let q = meta_query_sqlite_pragma_table_info!(for_table_name);
        let t = q.fetch_all(&mut **self).await?;
        Ok(t)
    }
    async fn get_tables(&mut self) -> Result<Vec<Self::TableInfo>, anyhow::Error> {
        Ok(vec![])
    }
    async fn get_metadata(&mut self) -> Result<Self::InformationSchema, anyhow::Error> {
        let tables = self::meta_fn_sqlite_txn_get_sqlite_master(&mut self, Some("table"), None, None, None).await?;
        // let d = HashMap::<String,TableMetadata>
        // let t = self::meta_fn_sqlite_txn_get_pragma_table_info_schema(&mut self).await?;
        dbg!(tables);

        todo!()
    }
}
