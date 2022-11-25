use crate::dao::dao_board;

use super::controller_common;
use warp::Reply;
pub async fn get_board(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::get_by_id(id).await, jwt_opt)
}

pub async fn get_all_boards(jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::get_all().await, jwt_opt)
}

pub async fn insert_board(
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::create(json_data).await, jwt_opt)
}

pub async fn update_board(
    id: i32,
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::update(id, json_data).await, jwt_opt)
}

pub async fn delete_board(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::delete(id).await, jwt_opt)
}