use warp::{Filter, Rejection, Reply};

use crate::{controller::controller_generic, util::util_authentication::auth_validator};
pub async fn get_generic_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_column = warp::path("generic");
    db_column
        .and(warp::get())
        .and(auth_validator("get_board".to_string()).await)
        .untuple_one()
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(controller_generic::get_generic)
        .or(db_column
            .and(warp::get())
            .and(warp::path("all"))
            .and(auth_validator("get_all_boards".to_string()).await)
            .untuple_one()
            .and(warp::path::end())
            .and_then(controller_generic::get_all_generics))
        .or(db_column
            .and(warp::post())
            .and(warp::path::end())
            .and(auth_validator("insert_board".to_string()).await)
            .untuple_one()
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and_then(controller_generic::insert_generic))
        // .or(db_column
        //     .and(warp::put())
        //     .and(auth_validator("update_board".to_string()).await)
        //     .untuple_one()
        //     .and(warp::path::param::<i32>())
        //     .and(warp::path::end())
        //     .and(warp::body::json())
        //     .and(warp::body::content_length_limit(1024 * 16))
        //     .and_then(controller_generic::update_generic))
        .or(db_column
            .and(warp::delete())
            .and(auth_validator("delete_board".to_string()).await)
            .untuple_one()
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and_then(controller_generic::delete_generic))
}
