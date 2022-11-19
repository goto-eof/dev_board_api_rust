use super::controller_common;
use crate::dao::dao_item;
use crate::structure::structures::SwapRequest;
use warp::Reply;

pub async fn get_item(id: i32) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_item::get_by_id(id).await)
}

pub async fn get_items() -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_item::get_all().await)
}

pub async fn get_by_parent_id(parent_id: i32) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_item::get_by_parent_id(parent_id).await)
}

pub async fn insert_item(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_item::create(json_data).await)
}

pub async fn update_item(
    id: i32,
    json_data: serde_json::Value,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_item::update(id, json_data).await)
}

pub async fn swap_items(swap_request: SwapRequest) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_item::swap(swap_request).await)
}

pub async fn delete_item(id: i32) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_item::delete(id).await)
}
