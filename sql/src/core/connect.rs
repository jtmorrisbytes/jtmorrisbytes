use std::{str::FromStr, sync::Arc};

use deadpool::managed::{Manager, Pool};
use tokio_postgres::{Client, Socket, tls::MakeTlsConnect};
use tokio_postgres_rustls::MakeRustlsConnect;

// performs setup tasks whenever a conn is created
pub fn on_con_init() {
    
}



// #[derive(Debug)]
pub struct SingleConnHandle {
    pub client: tokio_postgres::Client,
    pub socket_task: tokio::task::JoinHandle<Result<(), tokio_postgres::Error>>,
    // #[debug(skip)]
    pub tls: MakeRustlsConnect,
}

// type CF<'url> = impl Future<Output = std::io::Result<C>> + Send + use<'url>;
/// creates a single connection. useful for one offs. no validation just does the work
pub fn connect<'url>(
    url: &'url str,
) -> impl Future<Output = std::io::Result<SingleConnHandle>> + Send + use<'url> {
    
    let handle = tokio::runtime::Handle::try_current().or_else(|e| {
        Err(std::io::Error::new(
            std::io::ErrorKind::ConnectionAborted,
            format!("Runtime Error: you must call this fn in the context of a tokio runtime: {e}"),
        ))
    }).unwrap();
    async move {
        let c = rustls_client_config();
        let tls = MakeRustlsConnect::new(c);
        // let c = tokio_postgres::Config::from_str(url).unwrap();
        let (client,socket) = tokio_postgres::connect(&url, tls.clone())
        .await.map_err(|e|std::io::Error::from(std::io::ErrorKind::ConnectionRefused))?;

        let join = handle.spawn(socket);
        let c = SingleConnHandle {
            client,
            socket_task: join,
            tls: tls,
        };

        Ok(c)
    }
}

pub fn rustls_client_config() -> rustls::ClientConfig {
    let mut store = rustls::RootCertStore::empty();
    store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let native_result = rustls_native_certs::load_native_certs();
    // store.extend(native_result.certs.iter().cloned());
    let _ = store.add_parsable_certificates(native_result.certs);
    let store = Arc::new(store);
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(store)
        .with_no_client_auth();
    config
}


pub fn pooled<'url>(url: &'url str) -> Result<deadpool_postgres::Pool,Box<dyn std::error::Error>>{
    // 1. Your existing 'Monster' TLS setup
    let config = rustls_client_config();
    let tls = MakeRustlsConnect::new(config);

    // 2. Configure the Postgres connection
    let pg_config = tokio_postgres::Config::from_str(url)?;
    // 3. The Manager: It uses your 'tls' to create and recycle connections
    let manager_config = deadpool_postgres::ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast, // Minimal overhead
    };
    let manager = deadpool_postgres::Manager::from_config(pg_config, tls, manager_config);

    // 4. Build the Pool
    let pool = Pool::builder(manager)
        .max_size(20)
        .runtime(deadpool_postgres::Runtime::Tokio1)
        .build()?;
    Ok(pool)
}