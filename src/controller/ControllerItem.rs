use crate::{dao::DaoItem, Structs::SwapRequest};
use warp::Reply;

use super::ControllerCommon;

pub async fn get_item(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoItem::get_by_id(id).await)
}

pub async fn get_items() -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoItem::get_all().await)
}

pub async fn get_by_parent_id(parent_id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoItem::get_by_parent_id(parent_id).await)
}

pub async fn insert_item(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoItem::create(json_data).await)
}

pub async fn update_item(
    id: i32,
    json_data: serde_json::Value,
) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoItem::update(id, json_data).await)
}

pub async fn swap_items(swap_request: SwapRequest) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoItem::swap(swap_request).await)
}

pub async fn delete_item(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoItem::delete(id).await)
}
