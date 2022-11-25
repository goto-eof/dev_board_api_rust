use crate::dao::dao_role;

use super::controller_common;
use warp::Reply;
pub async fn get_role(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_role::get_by_id(id).await, jwt_opt)
}

pub async fn get_all_roles(jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_role::get_all().await, jwt_opt)
}

pub async fn insert_role(
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_role::create(json_data).await, jwt_opt)
}

pub async fn update_role(
    id: i32,
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_role::update(id, json_data).await, jwt_opt)
}

pub async fn delete_role(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_role::delete(id).await, jwt_opt)
}
