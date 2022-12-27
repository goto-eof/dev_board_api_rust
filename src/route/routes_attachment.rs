use crate::controller::controller_attachment;
use crate::util::util_authentication::auth_validator;
use warp::{Filter, Rejection, Reply};
pub async fn get_attachment_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    let db_column = warp::path("attachment");
    db_column
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(auth_validator("get_attachment".to_string()).await)
        .and_then(controller_attachment::get_attachment)
        .or(db_column
            .and(warp::get())
            .and(warp::path("get_by_item_id"))
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(auth_validator("get_attachment_by_item_id".to_string()).await)
            .and_then(controller_attachment::get_by_item_id))
        .or(db_column
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(auth_validator("insert_attachment".to_string()).await)
            .and_then(controller_attachment::insert_attachment))
        .or(db_column
            .and(warp::put())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(auth_validator("update_attachment".to_string()).await)
            .and_then(controller_attachment::update_attachment))
        .or(db_column
            .and(warp::delete())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(auth_validator("delete_attachment".to_string()).await)
            .and_then(controller_attachment::delete_attachment))
}
