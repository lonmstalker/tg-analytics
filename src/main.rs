extern crate core;

use teloxide::{prelude::*,
               dispatching::{
                   update_listeners::{webhooks},
                   dialogue::InMemStorage,
               },
               error_handlers::ErrorHandler,
};

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
    let pool = shared_config::init_db();
}
