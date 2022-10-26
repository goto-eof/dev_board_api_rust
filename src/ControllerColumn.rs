use crate::{
    DaoColumn::{self},
    ErrorStructs::DaoError,
};
use serde::Serialize;
use warp::{reply::json, Reply};

pub async fn get(id: i32) -> crate::GenericResult<impl Reply> {
    generate_response(DaoColumn::get_by_id(id).await)
}

pub async fn get_all() -> crate::GenericResult<impl Reply> {
    generate_response(DaoColumn::get_all().await)
}

pub async fn insert(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    generate_response(DaoColumn::create(json_data).await)
}

pub async fn update(id: i32, json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    generate_response(DaoColumn::update(id, json_data).await)
}

pub async fn delete(id: i32) -> crate::GenericResult<impl Reply> {
    generate_response(DaoColumn::delete(id).await)
}

fn generate_response<T: Serialize>(data: Result<T, DaoError>) -> crate::GenericResult<impl Reply> {
    match data {
        Ok(result) => Ok(json::<_>(&result)),
        Err(err) => Ok(json::<_>(&err)),
    }
}
