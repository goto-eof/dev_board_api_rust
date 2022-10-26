use crate::DaoColumn;
use warp::{reply::json, Reply};

pub async fn get(id: i32) -> crate::GenericResult<impl Reply> {
    let model = DaoColumn::get_by_id(id).await;
    Ok(json::<_>(&model))
}

pub async fn get_all() -> crate::GenericResult<impl Reply> {
    let model = DaoColumn::get_all().await;
    Ok(json::<_>(&model))
}

pub async fn insert(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    let model = DaoColumn::create(json_data).await;
    Ok(json::<_>(&model))
}

pub async fn update(id: i32, json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    let model = DaoColumn::update(id, json_data).await;
    Ok(json::<_>(&model))
}

pub async fn delete(id: i32) -> crate::GenericResult<impl Reply> {
    let result = DaoColumn::delete(id).await;
    Ok(json::<_>(&result))
}
