use crate::{controller::controller_permission, util::util_authentication::auth_validator};
use warp::{Filter, Rejection, Reply};

pub async fn get_permission_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    let db_column = warp::path("permission");
    db_column
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(auth_validator("get_permission".to_string()).await)
        .and_then(controller_permission::get_permission)
        .or(db_column
            .and(warp::get())
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and(auth_validator("get_permission_by_name".to_string()).await)
            .and_then(controller_permission::get_permission_by_name))
        .or(db_column
            .and(warp::get())
            .and(warp::path("all"))
            .and(warp::path::end())
            .and(auth_validator("get_all_permissions".to_string()).await)
            .and_then(controller_permission::get_all_permissions))
        .or(db_column
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(auth_validator("insert_permission".to_string()).await)
            .and_then(controller_permission::insert_permission))
        .or(db_column
            .and(warp::put())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(auth_validator("update_permission".to_string()).await)
            .and_then(controller_permission::update_permission))
        .or(db_column
            .and(warp::delete())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(auth_validator("delete_permission".to_string()).await)
            .and_then(controller_permission::delete_permission))
}
