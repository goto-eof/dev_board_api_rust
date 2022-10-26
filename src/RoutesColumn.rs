use mobc::Pool;
use mobc_postgres::{tokio_postgres::NoTls, PgConnectionManager};
use warp::{Filter, Rejection, Reply};

use crate::{with_db, ControllerColumn};

pub fn get_routes(// db_pool: Pool<PgConnectionManager<NoTls>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_column = warp::path("column");
    db_column
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(ControllerColumn::get)
        .or(db_column
            .and(warp::get())
            .and(warp::path::end())
            .and_then(ControllerColumn::get_all))
        .or(db_column
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(ControllerColumn::insert))
        .or(db_column
            .and(warp::put())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(ControllerColumn::update))
        .or(db_column
            .and(warp::delete())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and_then(ControllerColumn::delete))
}
