use warp::{Filter, Rejection, Reply};

use crate::{AuthenticationUtil::auth_validator, ControllerPermission};

pub async fn get_permission_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    let db_column = warp::path("permission");
    db_column
        .and(warp::get())
        .and(auth_validator("get_permission".to_string()).await)
        .untuple_one()
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(ControllerPermission::get_permission)
        .or(db_column
            .and(warp::get())
            .and(auth_validator("get_permission_by_name".to_string()).await)
            .untuple_one()
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and_then(ControllerPermission::get_permission_by_name))
        .or(db_column
            .and(warp::get())
            .and(auth_validator("get_all_permissions".to_string()).await)
            .untuple_one()
            .and(warp::path::end())
            .and_then(ControllerPermission::get_all_permissions))
        .or(db_column
            .and(warp::post())
            .and(auth_validator("insert_permission".to_string()).await)
            .untuple_one()
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(ControllerPermission::insert_permission))
        .or(db_column
            .and(warp::put())
            .and(auth_validator("update_permission".to_string()).await)
            .untuple_one()
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(ControllerPermission::update_permission))
        .or(db_column
            .and(warp::delete())
            .and(auth_validator("delete_permission".to_string()).await)
            .untuple_one()
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and_then(ControllerPermission::delete_permission))
}
