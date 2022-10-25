use crate::{database_config, error_manager, error_manager::Error::*, structs_column::*, DBPool};
use chrono::prelude::*;
use mobc_postgres::tokio_postgres;
use tokio_postgres::Row;

type Result<T> = std::result::Result<T, error_manager::Error>;

const TABLE: &str = "db_column_items";
const SELECT_FIELDS: &str = "ctm_id, ctm_name, created_at";

pub async fn fetch_all(db_pool: &DBPool, search: Option<String>) -> Result<Vec<DbColumnItems>> {
    let con = database_config::get_db_con(db_pool).await?;
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
    let con = database_config::get_db_con(db_pool).await?;
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
    let con = database_config::get_db_con(db_pool).await?;
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
    let con = database_config::get_db_con(db_pool).await?;
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
