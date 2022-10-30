use crate::{
    ControllerCommon,
    DaoItem::{self},
};
use warp::Reply;

pub async fn get(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoItem::get_by_id(id).await)
}

pub async fn get_all() -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoItem::get_all().await)
}

pub async fn get_by_parent_id(parent_id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoItem::get_by_parent_id(parent_id).await)
}

pub async fn insert(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoItem::create(json_data).await)
}

pub async fn update(id: i32, json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoItem::update(id, json_data).await)
}

pub async fn delete(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoItem::delete(id).await)
}
