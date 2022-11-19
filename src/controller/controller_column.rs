use super::controller_common;
use crate::dao::dao_column;
use crate::structure::structures::SwapRequest;
use warp::Reply;

pub async fn get_column(id: i32) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_column::get_by_id(id).await)
}

pub async fn get_all_columns() -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_column::get_all().await)
}
pub async fn get_all_columns_with_items() -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_column::get_all_with_items().await)
}
pub async fn insert_column(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_column::create(json_data).await)
}
pub async fn update_column(
    id: i32,
    json_data: serde_json::Value,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_column::update(id, json_data).await)
}

pub async fn swap_columns(swap_request: SwapRequest) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_column::swap(swap_request).await)
}

pub async fn delete_column(id: i32) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_column::delete(id).await)
}