use crate::dao::dao_board;

use super::controller_common;
use warp::Reply;
pub async fn get_board(id: i32) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::get_by_id(id).await)
}

pub async fn get_all_boards() -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::get_all().await)
}

pub async fn insert_board(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::create(json_data).await)
}

pub async fn update_board(
    id: i32,
    json_data: serde_json::Value,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::update(id, json_data).await)
}

pub async fn delete_board(id: i32) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::delete(id).await)
}
