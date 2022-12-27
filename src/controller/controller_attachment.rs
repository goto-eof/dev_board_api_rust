use super::controller_common;
use crate::dao::dao_attachment;
use warp::Reply;

pub async fn get_attachment(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_attachment::get_by_id(id).await, jwt_opt)
}

pub async fn get_by_item_id(
    parent_id: i32,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_attachment::get_by_item_id(parent_id).await, jwt_opt)
}

pub async fn insert_attachment(
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_attachment::create(json_data).await, jwt_opt)
}

pub async fn update_attachment(
    id: i32,
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_attachment::update(id, json_data).await, jwt_opt)
}

pub async fn delete_attachment(
    id: i32,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_attachment::delete(id, jwt_opt.clone()).await, jwt_opt)
}
