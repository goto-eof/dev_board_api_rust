use crate::{error, error::Error::*, mode::*, DBCon, DBPool};
use chrono::prelude::*;
use mobc::Pool;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls, Row};

type Result<T> = std::result::Result<T, error::Error>;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;
const INIT_SQL: &str = "./db.sql";
const TABLE: &str = "db_column_items";
const SELECT_FIELDS: &str = "ctm_id, ctm_name, created_at";

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let con = get_db_con(db_pool).await?;
    con.batch_execute(init_file.as_str())
        .await
        .map_err(DBInitError)?;
    Ok(())
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().await.map_err(DBPoolError)
}

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str("postgres://postgres:postgres@127.0.0.1:5432/postgres")?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

pub async fn fetch_all(db_pool: &DBPool, search: Option<String>) -> Result<Vec<DbColumnItems>> {
    let con = get_db_con(db_pool).await?;
    let where_clause = match search {
        Some(_) => "WHERE ctm_name like $1",
        None => "",
    };
    let query = format!(
        "SELECT {} FROM {} {} ORDER BY created_at DESC",
        SELECT_FIELDS, TABLE, where_clause
    );

    let q = match search {
        Some(v) => con.query(query.as_str(), &[&v]).await,
        None => con.query(query.as_str(), &[]).await,
    };
    let rows = q.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_item(&r)).collect())
}

pub async fn create(db_pool: &DBPool, body: DbColumnItemsRequest) -> Result<DbColumnItems> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (ctm_name) VALUES ($1) RETURNING *", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.ctm_name])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_item(&row))
}

pub async fn update(
    db_pool: &DBPool,
    id: i32,
    body: DbColumnItemsUpdateRequest,
) -> Result<DbColumnItems> {
    let con = get_db_con(db_pool).await?;
    let query = format!("UPDATE {} SET ctm_name = $1 WHERE ctm_id = $2", TABLE);
    println!("{}", &query);
    con.execute(query.as_str(), &[&body.ctm_name, &id])
        .await
        .map_err(DBQueryError)?;

    let query = format!("SELECT * FROM {} WHERE ctm_id = $1", TABLE);
    println!("{}", &query);
    let row = con
        .query_one(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_item(&row))
}

pub async fn delete(db_pool: &DBPool, id: i32) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE ctm_id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)
}

fn row_to_item(row: &Row) -> DbColumnItems {
    let ctm_id: i32 = row.get(0);
    let ctm_name: String = row.get(1);
    let created_at: DateTime<Utc> = row.get(2);
    DbColumnItems {
        ctm_id,
        ctm_name,
        created_at,
    }
}
