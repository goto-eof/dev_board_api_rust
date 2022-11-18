use crate::{
    ControllerCommon,
    DaoUser::{self},
};
use warp::Reply;

pub async fn get_user(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoUser::get_by_id(id).await)
}

pub async fn get_by_username(name: String) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoUser::get_by_username(name).await)
}

pub async fn get_all_users() -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoUser::get_all().await)
}

pub async fn insert_user(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoUser::create(json_data).await)
}

pub async fn update_user(
    id: i32,
    json_data: serde_json::Value,
) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoUser::update(id, json_data).await)
}

pub async fn delete_user(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoUser::delete(id).await)
}
