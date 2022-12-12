use crate::controller::controller_auth;
use crate::controller::controller_user;
use crate::util::util_authentication::auth_validator;
use warp::{Filter, Rejection, Reply};

pub async fn get_user_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_column = warp::path("user");
    db_column
        .and(warp::path("register"))
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(warp::body::content_length_limit(1024 * 16))
        .and_then(controller_auth::register)
        .or(db_column
            .and(warp::path("refreshToken"))
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and_then(controller_auth::refresh_jwt))
        .or(db_column
            .and(warp::path("login"))
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and_then(controller_auth::login))
        .or(db_column
            .and(warp::path("check_is_logged_in"))
            .and(warp::get())
            .and(warp::path::end())
            .and(auth_validator("check_is_logged_in".to_string()).await)
            .and_then(controller_auth::check_is_logged_in))
        .or(db_column
            .and(warp::get())
            .and(warp::path("get_user_by_id"))
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(auth_validator("get_user_by_id".to_string()).await)
            .and_then(controller_user::get_user_by_id))
        .or(db_column
            .and(warp::get())
            .and(warp::path("get_user"))
            .and(warp::path::end())
            .and(auth_validator("get_user".to_string()).await)
            .and_then(controller_user::get_user))
        .or(db_column
            .and(warp::get())
            .and(warp::path("get_by_username"))
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and(auth_validator("get_by_username".to_string()).await)
            .and_then(controller_user::get_by_username))
        .or(db_column
            .and(warp::get())
            .and(warp::path("all"))
            .and(warp::path::end())
            .and(auth_validator("get_all_users".to_string()).await)
            .and_then(controller_user::get_all_users))
        .or(db_column
            .and(warp::get())
            .and(warp::path("all-for-share"))
            .and(warp::path::end())
            .and(auth_validator("get_all_users_for_sharing".to_string()).await)
            .and_then(controller_user::get_all_users_for_sharing))
        .or(db_column
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(auth_validator("insert_user".to_string()).await)
            .and_then(controller_user::insert_user))
        .or(db_column
            .and(warp::put())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(auth_validator("update_user".to_string()).await)
            .and_then(controller_user::update_user))
        .or(db_column
            .and(warp::delete())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(auth_validator("delete_user".to_string()).await)
            .and_then(controller_user::delete_user))
}
