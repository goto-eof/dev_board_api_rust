use crate::controller::controller_message;
use crate::util::util_authentication::auth_validator;
use warp::{Filter, Rejection, Reply};
pub async fn get_message_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_column = warp::path("message");
    db_column
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(auth_validator("get_message".to_string()).await)
        .and_then(controller_message::get_message)
        .or(db_column
            .and(warp::get())
            .and(warp::path("get_by_item_id"))
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(auth_validator("get_by_item_id".to_string()).await)
            .and_then(controller_message::get_by_item_id))
        .or(db_column
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(auth_validator("insert_message".to_string()).await)
            .and_then(controller_message::insert_message))
        .or(db_column
            .and(warp::put())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(auth_validator("update_message".to_string()).await)
            .and_then(controller_message::update_message))
        .or(db_column
            .and(warp::delete())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(auth_validator("delete_message".to_string()).await)
            .and_then(controller_message::delete_message))
}
