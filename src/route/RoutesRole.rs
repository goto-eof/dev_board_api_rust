use crate::controller::ControllerRole;
use crate::util::AuthenticationUtil::auth_validator;
use warp::{Filter, Rejection, Reply};

pub async fn get_role_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_column = warp::path("role");
    db_column
        .and(warp::get())
        .and(auth_validator("get_role".to_string()).await)
        .untuple_one()
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(ControllerRole::get_role)
        .or(db_column
            .and(warp::get())
            .and(auth_validator("get_all_roles".to_string()).await)
            .untuple_one()
            .and(warp::path::end())
            .and_then(ControllerRole::get_all_roles))
        .or(db_column
            .and(warp::post())
            .and(auth_validator("insert_role".to_string()).await)
            .untuple_one()
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(ControllerRole::insert_role))
        .or(db_column
            .and(warp::put())
            .and(auth_validator("update_role".to_string()).await)
            .untuple_one()
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(ControllerRole::update_role))
        .or(db_column
            .and(warp::delete())
            .and(auth_validator("delete_role".to_string()).await)
            .untuple_one()
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and_then(ControllerRole::delete_role))
}
