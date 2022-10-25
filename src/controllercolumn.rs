use std::str::FromStr;

use crate::{DBPool, DaoColumn, StructColumns::*};
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

pub async fn get_by_id(query: SearchIntQuery, db_pool: DBPool) -> crate::GenericResult<impl Reply> {
    debug!("SEARCH QUERY: {:?}", &query);
    let model = DaoColumn::get_by_id(&db_pool, query.id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<_>(&DbColumnItemsResponse::of(model)))
}

pub async fn list_column_items_handler(db_pool: DBPool) -> crate::GenericResult<impl Reply> {
    let model = DaoColumn::fetch_all(&db_pool)
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
) -> crate::GenericResult<impl Reply> {
    Ok(json(&DbColumnItemsResponse::of(
        DaoColumn::create(&db_pool, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn update_column_items_handler(
    id: i32,
    body: DbColumnItemsUpdateRequest,
    db_pool: DBPool,
) -> crate::GenericResult<impl Reply> {
    Ok(json(&DbColumnItemsResponse::of(
        DaoColumn::update(&db_pool, id, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn delete_column_items_handler(
    id: i32,
    db_pool: DBPool,
) -> crate::GenericResult<impl Reply> {
    DaoColumn::delete(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}
