use std::str::FromStr;

use crate::{daocolumn, database_config, error_manager::Error::*, structs_column::*, DBPool};
use log::debug;
use serde_derive::Deserialize;
use warp::{http::StatusCode, reject, reply::json, Reply};

pub struct MioError;
impl FromStr for SearchIntQuery {
    type Err = MioError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "error" => Err(MioError),
            _ => {
                let id = s.parse::<i32>();
                Ok(SearchIntQuery {
                    id: Some(id.unwrap()),
                })
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SearchIntQuery {
    id: Option<i32>,
}

pub async fn health_handler(db_pool: DBPool) -> crate::Result<impl Reply> {
    let db = database_config::get_db_con(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;
    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}

pub async fn get_by_id(query: SearchIntQuery, db_pool: DBPool) -> crate::Result<impl Reply> {
    debug!("SEARCH QUERY: {:?}", &query);
    let model = daocolumn::get_by_id(&db_pool, query.id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<_>(&DbColumnItemsResponse::of(model)))
}

pub async fn list_column_items_handler(db_pool: DBPool) -> crate::Result<impl Reply> {
    let model = daocolumn::fetch_all(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &model
            .into_iter()
            .map(|t| DbColumnItemsResponse::of(t))
            .collect(),
    ))
}

pub async fn create_column_items_handler(
    body: DbColumnItemsRequest,
    db_pool: DBPool,
) -> crate::Result<impl Reply> {
    Ok(json(&DbColumnItemsResponse::of(
        daocolumn::create(&db_pool, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn update_column_items_handler(
    id: i32,
    body: DbColumnItemsUpdateRequest,
    db_pool: DBPool,
) -> crate::Result<impl Reply> {
    Ok(json(&DbColumnItemsResponse::of(
        daocolumn::update(&db_pool, id, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn delete_column_items_handler(id: i32, db_pool: DBPool) -> crate::Result<impl Reply> {
    daocolumn::delete(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}
