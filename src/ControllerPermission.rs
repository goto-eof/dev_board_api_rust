use crate::{ControllerCommon, DaoPermission};
use warp::Reply;

pub async fn get_permission(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoPermission::get_by_id(id).await)
}

pub async fn get_permission_by_name(name: String) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoPermission::get_by_name(&name).await)
}

pub async fn get_all_permissions() -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoPermission::get_all().await)
}

pub async fn insert_permission(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoPermission::create(json_data).await)
}

pub async fn update_permission(
    id: i32,
    json_data: serde_json::Value,
) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoPermission::update(id, json_data).await)
}

pub async fn delete_permission(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoPermission::delete(id).await)
}
