use crate::{DBPool, DaoColumn, StructColumns::*};
use log::debug;
use warp::{http::StatusCode, reject, reply::json, Reply};

pub async fn get_by_id(id: i32, db_pool: DBPool) -> crate::GenericResult<impl Reply> {
    debug!("SEARCH QUERY: {:?}", id);
    let model = DaoColumn::get_by_id(&db_pool, id)
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
