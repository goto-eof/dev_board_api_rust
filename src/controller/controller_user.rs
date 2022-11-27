use crate::dao::dao_user;

use super::controller_common;
use warp::Reply;

pub async fn get_user(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_user::get_by_id(id).await, jwt_opt)
}

pub async fn get_by_username(
    name: String,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_user::get_by_username(name).await, jwt_opt)
}

pub async fn get_all_users(jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_user::get_all().await, jwt_opt)
}

pub async fn get_all_users_for_sharing(
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(
        dao_user::get_all_for_sharing(jwt_opt.clone()).await,
        jwt_opt,
    )
}

pub async fn insert_user(
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_user::create(json_data).await, jwt_opt)
}

pub async fn update_user(
    id: i32,
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_user::update(id, json_data).await, jwt_opt)
}

pub async fn delete_user(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_user::delete(id).await, jwt_opt)
}
