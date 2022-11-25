use crate::controller::controller_item;
use crate::util::util_authentication::auth_validator;
use warp::{Filter, Rejection, Reply};
pub async fn get_item_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_column = warp::path("item");
    db_column
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(auth_validator("get_item".to_string()).await)
        .and_then(controller_item::get_item)
        .or(db_column
            .and(warp::get())
            .and(warp::path("parent"))
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(auth_validator("get_by_parent_id".to_string()).await)
            .and_then(controller_item::get_by_parent_id))
        .or(db_column
            .and(warp::get())
            .and(warp::path("all"))
            .and(warp::path::end())
            .and(auth_validator("get_items".to_string()).await)
            .and_then(controller_item::get_items))
        .or(db_column
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(auth_validator("insert_item".to_string()).await)
            .and_then(controller_item::insert_item))
        .or(db_column
            .and(warp::put())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(auth_validator("update_item".to_string()).await)
            .and_then(controller_item::update_item))
        .or(db_column
            .and(warp::put())
            .and(warp::path("swap"))
            .and(warp::path::end())
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and(auth_validator("swap_items".to_string()).await)
            .and_then(controller_item::swap_items))
        .or(db_column
            .and(warp::delete())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(auth_validator("delete_item".to_string()).await)
            .and_then(controller_item::delete_item))
}
