use once_cell::sync::{OnceCell};
use deadpool_postgres::{Pool, Client};

pub static DB: OnceCell<Pool> = OnceCell::new();