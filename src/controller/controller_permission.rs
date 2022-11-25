use crate::dao::dao_permission;

use super::controller_common;
use warp::Reply;
pub async fn get_permission(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_permission::get_by_id(id).await, jwt_opt)
}

pub async fn get_permission_by_name(
    name: String,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_permission::get_by_name(&name).await, jwt_opt)
}

pub async fn get_all_permissions(jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_permission::get_all().await, jwt_opt)
}

pub async fn insert_permission(
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_permission::create(json_data).await, jwt_opt)
}

pub async fn update_permission(
    id: i32,
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_permission::update(id, json_data).await, jwt_opt)
}

pub async fn delete_permission(
    id: i32,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_permission::delete(id).await, jwt_opt)
}
