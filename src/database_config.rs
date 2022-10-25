use crate::{error_manager, error_manager::Error::*, DBCon, DBPool};
use log::debug;
use mobc::Pool;

use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls};
type Result<T> = std::result::Result<T, error_manager::Error>;

const INIT_SQL: &str = "./db.sql";
const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().await.map_err(DBPoolError)
}

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let con = get_db_con(db_pool).await?;
    con.batch_execute(init_file.as_str())
        .await
        .map_err(DBInitError)?;
    Ok(())
}

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let db_uri = std::env::var("DB_URI");
    let connection_uri;
    if db_uri.is_ok() {
        connection_uri = db_uri.unwrap();
    } else {
        connection_uri = "postgres://postgres:postgres@postgres_service:5432/postgres".to_string();
    }
    debug!("trying to connect to db on uri: {}", &connection_uri);
    let config = Config::from_str(&connection_uri)?;
    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}
