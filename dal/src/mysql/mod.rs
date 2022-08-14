use mysql_async::{OptsBuilder, Pool, Conn, Transaction, TxOpts};
use rand::Rng;
use tracing::{instrument, trace};
use thiserror::Error;
use serde::{Serialize, Deserialize};

mod device;
mod user;

pub use device::*;
pub use user::*;

mod migrations {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub type MysqlResult<T> = Result<T, MysqlError>;

#[derive(Debug, Error)]
pub enum MysqlError {
    #[error("{0}")]
    Refinery(#[from] refinery::Error),
    #[error("{0}")]
    Mysql(#[from] mysql_async::Error),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MysqlConfig {
    pub host: String,
    pub db: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct Mysql(Pool);

impl Mysql {
    #[instrument(skip(self))]
    pub(crate) async fn get_conn(&self) -> MysqlResult<Conn> {
        trace!("Aquiring connection");
        self.0.get_conn().await.map_err(|e| e.into())
    }

    #[instrument(skip(self))]
    pub(crate) async fn start_transaction(&self) -> MysqlResult<Transaction<'_>> {
        trace!("Starting transaction");
        self.0.start_transaction(TxOpts::default()).await.map_err(|e| e.into())
    }

    #[instrument(skip(config))]
    pub async fn init(config: MysqlConfig) -> MysqlResult<Self> {
        trace!("Initializing Mysql");
        let opts = OptsBuilder::default()
            .ip_or_hostname(config.host)
            .db_name(Some(config.db))
            .user(Some(config.user))
            .pass(Some(config.password));

        let mut pool = Pool::new(opts);

        trace!("Running migrations");
        migrations::migrations::runner()
            .set_migration_table_name("__miniboss_migrations")
            .run_async(&mut pool).await?;

        Ok(Self(pool))
    }
}

pub(crate) fn generate_id() -> String {
    rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(32).map(char::from).collect()
}
