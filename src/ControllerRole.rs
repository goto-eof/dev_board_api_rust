use crate::{ControllerCommon, DaoRole};
use warp::Reply;

pub async fn get_role(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoRole::get_by_id(id).await)
}

pub async fn get_all_roles() -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoRole::get_all().await)
}

pub async fn insert_role(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoRole::create(json_data).await)
}

pub async fn update_role(
    id: i32,
    json_data: serde_json::Value,
) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoRole::update(id, json_data).await)
}

pub async fn delete_role(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoRole::delete(id).await)
}
