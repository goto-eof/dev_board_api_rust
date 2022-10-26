use crate::DaoColumn::{self, DaoError};
use serde::Serialize;
use warp::{reply::json, Reply};

pub async fn get(id: i32) -> crate::GenericResult<impl Reply> {
    generate_json(DaoColumn::get_by_id(id).await)
}

pub async fn get_all() -> crate::GenericResult<impl Reply> {
    generate_json(DaoColumn::get_all().await)
}

pub async fn insert(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    generate_json(DaoColumn::create(json_data).await)
}

pub async fn update(id: i32, json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    generate_json(DaoColumn::update(id, json_data).await)
}

pub async fn delete(id: i32) -> crate::GenericResult<impl Reply> {
    generate_json(DaoColumn::delete(id).await)
}

fn generate_json<T: Serialize>(data: Result<T, DaoError>) -> crate::GenericResult<impl Reply> {
    match data {
        Ok(result) => Ok(json::<_>(&result)),
        Err(err) => Ok(json::<_>(&err)),
    }
}
