extern crate core;

use std::env;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use native_tls::TlsConnector;
use postgres_native_tls::{MakeTlsConnector};
use teloxide::{prelude::*,
               dispatching::{
                   update_listeners::{webhooks},
                   dialogue::InMemStorage,
               },
               error_handlers::ErrorHandler,
};
use tokio::task::JoinHandle;

#[path = "exception/error_handler.rs"]
mod error_handler;

#[path = "config/shared_config.rs"]
mod shared_config;

#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    pretty_env_logger::init();
    log::info!("start bot");

    let bot = Bot::from_env();

    // Open database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable missing");

    let connector = TlsConnector::builder()
        // .add_root_certificate(cert)
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let connector = MakeTlsConnector::new(connector);

    let pg_config = database_url.parse::<tokio_postgres::Config>().expect("database url wrong");
    let mgr_config = ManagerConfig { recycling_method: RecyclingMethod::Fast };
    let mgr = Manager::from_config(pg_config, connector, mgr_config);
    let pool = Pool::builder(mgr).max_size(16).build().unwrap();

    test_db_connection(pool.clone());

    match shared_config::DB.set(pool) {
        Ok(_) => log::info!("db connected"),
        _ => panic!("db connect error")
    }
}

fn test_db_connection(test_pool: Pool) -> JoinHandle<()> {
    tokio::spawn(async move {
        if let Err(e) = test_pool.get().await {
            log::error!("Database error: {}", e);
            panic!();
        }
    })
}