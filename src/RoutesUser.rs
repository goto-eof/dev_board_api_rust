use warp::{Filter, Rejection, Reply};

use crate::{AuthenticationUtil::auth_validator, ControllerAuth};

pub async fn get_user_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_column = warp::path("user");
    db_column
        .and(warp::path("register"))
        .and(warp::post())
        .and(warp::path::end())
        // .and(auth_validator("register".to_string()).await)
        // .untuple_one()
        .and(warp::body::json())
        .and(warp::body::content_length_limit(1024 * 16))
        .and_then(ControllerAuth::register)
        .or(db_column
            .and(warp::path("login"))
            .and(warp::post())
            .and(warp::path::end())
            // .and(auth_validator("login".to_string()).await)
            // .untuple_one()
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and_then(ControllerAuth::login))
}
