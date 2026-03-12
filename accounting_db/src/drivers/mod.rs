use std::sync::Arc;

#[cfg(feature="postgresql")]
pub(super) mod postgres;
#[cfg(feature="mysql")]
pub(super) mod mysql;
#[cfg(feature="sqlite")]
pub(super) mod sqlite;

// the driver contracts 'traits' to make it work with any driver
// any specific behavior should be defined here and then implemented in these subcrates

/*
    example:
    Trait Obj:
        performs sql functions
        A() -> T
        B() -> Y
        C() -> Z
    then each function gets implemented in the subcrates in this module for each specific sql Exectutor
    ex:
    impl Obj for PgPool {
        ....
    }
    which allows

    fn run_query(executor: [any executor that implements Obj]) -> {
        //cast executor as contract trait Obj then call its method
        (t as Obj).A()
    }
    // can also store as registry types
    DashMap<id,Arc<dyn Obj>>

    make sure you require Send + Sync in the trait definitions
    otherwise the compiler will complain that your trait object cannot be sent accross threads safely
*/

#[async_trait::async_trait]
/// the user implementation in this database crate is 
/// is implemented via multiple databases. all user interactions go through
/// this trait.
pub trait UserTenantStore: Send + Sync {
    // async fn save_passkey(&self){}
}





/// Aquires a connection. Mysql, and postgres are single connections with this method, sqlite connections are POOLED, as SqliteConnection is not Send + Sync
pub async fn connect_tenant_any_url<S: AsRef<str>>(database_url:S) -> Result<Arc<dyn UserTenantStore>,anyhow::Error>
{
    sqlx::any::install_default_drivers();

    let url = database_url.as_ref();
    #[cfg(feature = "sqlite")] {
        // NOTE: Sqlite connections are NOT Send + Sync, so POOLING IS REQUIRED
        if url.starts_with("sqlite:///") {
            return Ok(Arc::new(sqlite::connect_pool_url(url).await?) as Arc<dyn UserTenantStore>)
        }
    }
    #[cfg(feature = "postgresql")] {
        if url.starts_with("postgres://") || url.starts_with("postgresql://") {
            return postgres::connect_url(url).await.map(|c| Arc::new(c) as Arc<dyn UserTenantStore>)
        }
    }
    #[cfg(feature = "mysql")] {
        if url.starts_with("mysql://") {
            return mysql::connect_url(url).await.map(|c| Arc::new(c) as Arc<dyn UserTenantStore>)
        }
    }

    return Err(anyhow::Error::msg("Database crate compiled without any drivers available or invalid connection url. unable to make a connection"))
    
}
/// Aquires a Pool with default options. Mysql, and postgres are POOLED connections with this method, sqlite connections are POOLED, as SqliteConnection is not Send + Sync
pub async fn connect_tenant_any_pool_url<S: AsRef<str>>(database_url:S) -> Result<Arc<dyn UserTenantStore>,anyhow::Error>
{
    sqlx::any::install_default_drivers();

    let url = database_url.as_ref();
    #[cfg(feature = "sqlite")] {
        if url.starts_with("sqlite:///") {
            return Ok(Arc::new(sqlite::connect_pool_url(url).await?) as Arc<dyn UserTenantStore>)
        }
    }
    #[cfg(feature = "postgresql")] {
        if url.starts_with("postgres://") || url.starts_with("postgresql://") {
            return postgres::connect_pool_url(url).await.map(|c| Arc::new(c) as Arc<dyn UserTenantStore>)
        }
    }
    #[cfg(feature = "mysql")] {
        if url.starts_with("mysql://") {
            return mysql::connect_pool_url(url).await.map(|c| Arc::new(c) as Arc<dyn UserTenantStore>)
        }
    }

    return Err(anyhow::Error::msg("Database crate compiled without any drivers available or invalid connection url. unable to make a connection"))
    
}


