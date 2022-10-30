use warp::{Filter, Rejection, Reply};

use crate::ControllerItem;

pub fn get_item_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_column = warp::path("item");
    db_column
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(ControllerItem::get)
        .or(db_column
            .and(warp::get())
            .and(warp::path("parent"))
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and_then(ControllerItem::get_by_parent_id))
        .or(db_column
            .and(warp::get())
            .and(warp::path::end())
            .and_then(ControllerItem::get_all))
        .or(db_column
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(ControllerItem::insert))
        .or(db_column
            .and(warp::put())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(ControllerItem::update))
        .or(db_column
            .and(warp::delete())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and_then(ControllerItem::delete))
}
