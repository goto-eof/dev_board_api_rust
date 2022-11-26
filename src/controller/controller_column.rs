use super::controller_common::generate_response;
use crate::dao::dao_column;
use crate::structure::structure::SwapRequest;
use warp::Reply;

pub async fn get_column(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    generate_response(dao_column::get_by_id(id).await, jwt_opt)
}

pub async fn get_all_columns(jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    generate_response(dao_column::get_all().await, jwt_opt)
}
pub async fn get_all_columns_with_items(
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    generate_response(dao_column::get_all_with_items().await, jwt_opt)
}
pub async fn insert_column(
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    generate_response(
        dao_column::create(json_data, jwt_opt.clone()).await,
        jwt_opt,
    )
}
pub async fn update_column(
    id: i32,
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    generate_response(dao_column::update(id, json_data).await, jwt_opt)
}

pub async fn swap_columns(
    swap_request: SwapRequest,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    generate_response(dao_column::swap(swap_request).await, jwt_opt)
}

pub async fn delete_column(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    generate_response(dao_column::delete(id).await, jwt_opt)
}
