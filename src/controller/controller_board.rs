use crate::dao::dao_board;

use super::controller_common;
use warp::Reply;

pub async fn get_board(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::get_by_id(id, jwt_opt.clone()).await, jwt_opt)
}

pub async fn get_board_with_all_data(
    id: i32,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(
        dao_board::get_by_id_all(id, jwt_opt.clone()).await,
        jwt_opt,
    )
}

pub async fn archive(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::archive(id, jwt_opt.clone()).await, jwt_opt)
}

pub async fn shared_with(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::shared_with(id, jwt_opt.clone()).await, jwt_opt)
}

pub async fn share_board(
    board_id: i32,
    user_id: i32,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(
        dao_board::share(board_id, user_id, jwt_opt.clone()).await,
        jwt_opt,
    )
}

pub async fn unshare_board(
    board_id: i32,
    user_id: i32,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(
        dao_board::unshare(board_id, user_id, jwt_opt.clone()).await,
        jwt_opt,
    )
}

pub async fn get_all_boards(jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::get_all(jwt_opt.clone()).await, jwt_opt)
}

pub async fn board_is_shared_with(
    board_id: i32,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(
        dao_board::board_is_shared_with(board_id, jwt_opt.clone()).await,
        jwt_opt,
    )
}

pub async fn insert_board(
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(
        dao_board::create(json_data, jwt_opt.clone()).await,
        jwt_opt,
    )
}

pub async fn update_board(
    id: i32,
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(
        dao_board::update(id, json_data, jwt_opt.clone()).await,
        jwt_opt,
    )
}

pub async fn delete_board(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_board::delete(id, jwt_opt.clone()).await, jwt_opt)
}
