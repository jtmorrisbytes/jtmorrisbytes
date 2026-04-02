// may the gods have mercy on my soul....

// yes. we are importing our core crate requirements for our recursive build
// LOL ! JT

use std::{collections::BTreeMap, str::FromStr};

use postgres_types::{FromSql, ToSql};
use tokio::io::AsyncWriteExt;


// this is the thing that people build entire frameworks for
pub fn db_generate<'client>(
    c: &'client mut tokio_postgres::Client,
) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + use<'client> {
    async move {
        // determine the root of the workspace
        let workspace_root = super::workspace_root()?;
        // the scripts dir is at the root of the crate
        // load the correct script
        let workspace_root = std::path::PathBuf::from(workspace_root);
        let p = workspace_root
            .join("sql_scripts")
            .join("db_tables.sql")
            .canonicalize()?;
        let sql = tokio::fs::read_to_string(&p).await?;
        // start a transaction, read only
        let t = c
            .build_transaction()
            .deferrable(true)
            .isolation_level(tokio_postgres::IsolationLevel::ReadCommitted)
            .read_only(true)
            .start()
            .await?;
        let args: &[&(dyn ToSql + Sync)] = &[];
        let rs = t.query(&sql,args).await?;
        let mut map: BTreeMap<String, Vec<_>> = BTreeMap::new();
        
        for r in rs {
            let t = InfoSchemaTableColumn {
                table_catalog:r.get(0),
                table_schema: r.get(1),
                table_name:r.get(2),
                table_type:r.get(3),
                column_name:r.get(4),
                data_type: r.get(5),
                udt_name: r.get(6),
                is_nullable: r.get(7)
            };
            map.entry(t.table_name.clone()).or_default().push(t);         
            // println!("cargo:warning={tc},{ts},{tn},{t_ty}");
        }

        t.commit().await?;
        let p = workspace_root.join("src").join("sql").join("generated.rs");
        let mut f = tokio::fs::File::create(p).await?;
        let mut tkns = String::new();
        for (table_name,c) in map {
            let struct_name = quote::format_ident!("{}",table_name);

            let fields_idents :Vec<_> = c.iter().map(|c|{
                let i = quote::format_ident!("{}",c.column_name);
                let n = c.column_name.as_str();
                quote::quote! {#i:row.get(#n)}
                
            }).collect();
            let fields_idents_ty: Vec<_> = c.iter().map(|c|{
                let col_name = quote::format_ident!("{}",c.column_name);
                let ty = produce_rust_type_from_pg_type_and_udtname_as_tokenstream(&c.data_type,&c.udt_name);
                let ty = {if c.is_nullable == 1 {quote::quote!{Option<#ty>}} else {ty}};
                quote::quote! {#col_name:#ty,}
            }).collect();
            let tkn = quote::quote! {
                #[derive(Debug,Clone,PartialEq)]
                #[allow(non_camel_case_types)]
                pub struct #struct_name{#(#fields_idents_ty)*}

            }.to_string();
            tkns = tkns + &tkn;
            
            let tkn = quote::quote! {
                impl std::convert::From<tokio_postgres::Row> for #struct_name {
                    fn from(row: tokio_postgres::Row) -> Self {
                        Self {
                            #(#fields_idents),*
                        }
                    }
                }
            }.to_string();
            tkns = tkns + &tkn;

        }
        
        f.write_all(tkns.as_bytes()).await?;


        // let dbg = format!("{map:?}");
        // f.write_all(dbg.as_bytes()).await?;
        Ok(())
    }
}



fn produce_rust_type_from_pg_type_and_udtname_as_tokenstream(pg_type: &str,udt_name: &str) -> proc_macro2::TokenStream {
    match pg_type {
                    "uuid" => proc_macro2::TokenStream::from_str("uuid::Uuid").unwrap(),
                    "text" => proc_macro2::TokenStream::from_str("String").unwrap(),
                    "bytea"=> proc_macro2::TokenStream::from_str("Vec<u8>").unwrap(),
                    "integer"=> proc_macro2::TokenStream::from_str("i32").unwrap(),
                    "boolean" => proc_macro2::TokenStream::from_str("bool").unwrap(),
                    "character varying" => proc_macro2::TokenStream::from_str("String").unwrap(),
                    // chrono offsetdatetime or something
                    "timestamp with time zone" => proc_macro2::TokenStream::from_str("i64").unwrap(),
                    "ARRAY"=> {
                        match udt_name {
                            "_text" => proc_macro2::TokenStream::from_str("Vec<String>").unwrap(),
                            other @ _ =>{
                                panic!("other_udt: {other}");
                            }
                        }
                    },
                    other @ _ => {
                        panic!("other:{}",other);
                    }

                }
}


#[derive(FromSql,Debug)]
struct InfoSchemaTableColumn {
    table_catalog:String,
    table_schema:String,
    table_name:String,
    table_type:String,
    column_name:String,
    data_type:String,
    udt_name:String,
    is_nullable:i32
}
