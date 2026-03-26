use postgres_types::FromSql;

// this crate to allow us to ask for the state of the database at runtime
#[derive(FromSql)]
struct ColumnInfo {
    pub table: String,
    pub name: String,
    pub ty: String,
    pub ordinal_position: i32,
    pub is_nullable: bool,
    pub udt_name: String,
}
fn get_column_info_by_name<'t1, 't2, 'tn, 'cn>(
    t: &'t1 tokio_postgres::Transaction<'t2>,
    table: &'tn str,
    column_name: &'cn str,
) -> impl Future<Output = Result<Option<ColumnInfo>, Box<dyn std::error::Error>>> + use<'t1, 't2, 'tn, 'cn>
{
    let arg1 = format!("%{table}%");
    let arg2 = format!("%{column_name}%");
    
    async move {
        let args: &[&(dyn postgres_types::ToSql + Sync)] = &[&arg1, &arg2];
        let r = t
            .query_opt(
                r#"SELECT "ordinal_position", "data_type","is_nullable","udt_name","table_name","column_name"
            from information_schema.columns
                  WHERE table_name LIKE $1
                  and column_name LIKE $2
                  ORDER BY ordinal_position ASC
                  LIMIT 1"#,
                &args,
            )
            .await?;

        if r.is_none() {
            return Ok(None);
        }
        let r = r.unwrap();
        Ok(Some(
            ColumnInfo {
                ordinal_position: r.get::<_, i32>(0),
                ty: r.get::<_, String>(1),
                is_nullable: r.get::<_, bool>(2),
                udt_name:  r.get::<_, String>(3),
                table: r.get::<_, String>(4),
                name: r.get(5)
            }
        ))
    }
}
