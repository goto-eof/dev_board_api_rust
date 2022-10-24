use crate::{dao_column, dbconfig, error::Error::*, mode::*, DBPool, Result};
use serde_derive::Deserialize;
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Deserialize)]
pub struct SearchQuery {
    search: Option<String>,
}

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply> {
    let db = dbconfig::get_db_con(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;
    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}

pub async fn list_column_items_handler(query: SearchQuery, db_pool: DBPool) -> Result<impl Reply> {
    let todos = dao_column::fetch_all(&db_pool, query.search)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &todos
            .into_iter()
            .map(|t| DbColumnItemsResponse::of(t))
            .collect(),
    ))
}

pub async fn create_column_items_handler(
    body: DbColumnItemsRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&DbColumnItemsResponse::of(
        dao_column::create(&db_pool, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn update_column_items_handler(
    id: i32,
    body: DbColumnItemsUpdateRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&DbColumnItemsResponse::of(
        dao_column::update(&db_pool, id, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn delete_column_items_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    dao_column::delete(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}
