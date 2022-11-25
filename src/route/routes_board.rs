use warp::{Filter, Rejection, Reply};

use crate::{controller::controller_board, util::util_authentication::auth_validator};
pub async fn get_board_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_column = warp::path("board");
    db_column
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(auth_validator("get_board".to_string()).await)
        .and_then(controller_board::get_board)
        .or(db_column
            .and(warp::get())
            .and(warp::path("all"))
            .and(warp::path::end())
            .and(auth_validator("get_all_boards".to_string()).await)
            .and_then(controller_board::get_all_boards))
        .or(db_column
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and(auth_validator("insert_board".to_string()).await)
            .and_then(controller_board::insert_board))
        .or(db_column
            .and(warp::put())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and(auth_validator("update_board".to_string()).await)
            .and_then(controller_board::update_board))
        .or(db_column
            .and(warp::delete())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(auth_validator("delete_board".to_string()).await)
            .and_then(controller_board::delete_board))
}
