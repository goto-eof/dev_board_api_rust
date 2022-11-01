use crate::{
    ControllerCommon,
    DaoColumn::{self},
    Structs::SwapRequest,
};
use warp::Reply;

pub async fn get(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoColumn::get_by_id(id).await)
}

pub async fn get_all() -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoColumn::get_all().await)
}

pub async fn insert(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoColumn::create(json_data).await)
}

pub async fn update(id: i32, json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoColumn::update(id, json_data).await)
}

pub async fn swap(swap_request: SwapRequest) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoColumn::swap(swap_request).await)
}

pub async fn delete(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoColumn::delete(id).await)
}
