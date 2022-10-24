use deadpool_postgres::{Pool, Client, ManagerConfig, RecyclingMethod, Manager};
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use serde::Deserialize;
use crate::shared_config::convert::ConvertProperties;

pub fn init_db() -> Pool {
    let connector = TlsConnector::builder()
        // .add_root_certificate(cert)
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let connector = MakeTlsConnector::new(connector);

    let pg_config = envy::prefixed("app_")
        .from_env::<AppProperties>()
        .unwrap()
        .convert_to_config();

    let mgr_config = ManagerConfig { recycling_method: RecyclingMethod::Fast };
    let mgr = Manager::from_config(pg_config, connector, mgr_config);

    let pool = Pool::builder(mgr)
        .max_size(16)
        .build()
        .unwrap();
    test_db_connection(&pool);

    return pool;
}

async fn test_db_connection(pool: &Pool) {
    if let Err(e) = pool.get().await {
        log::error!("Database error: {}", e);
        panic!();
    } else {
        log::info!("Database validated");
    }
}

#[derive(Deserialize)]
struct AppProperties {
    #[serde(default = "database_host")]
    database_host: String,

    #[serde(default = "database_user")]
    database_user: String,

    #[serde(default = "database_password")]
    database_password: String,

    #[serde(default = "database_port")]
    database_port: u16,

    #[serde(default = "database_name")]
    database_name: String,
}

fn database_port() -> u16 {
    return 5432;
}

fn database_name() -> String {
    return "tg_analytics".to_string(); // because here must be not owned
}

fn database_host() -> String {
    return "postgresql://localhost:5432".to_string();
}

fn database_user() -> String {
    return "tg_analytics".to_string();
}

fn database_password() -> String {
    return "tg_analytics".to_string();
}

mod convert {
    use tokio_postgres::Config;
    use crate::shared_config::AppProperties;

    pub trait ConvertProperties {
        fn convert_to_config(self) -> Config;
    }

    impl ConvertProperties for AppProperties {
        fn convert_to_config(self) -> Config {
            let mut config = Config::new();

            config.host(&*self.database_host);
            config.port(self.database_port);
            config.user(&*self.database_user);
            config.password(self.database_password);
            config.dbname(&*self.database_name);

            config
        }
    }
}
