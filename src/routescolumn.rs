use mobc::Pool;
use mobc_postgres::{tokio_postgres::NoTls, PgConnectionManager};
use warp::{Filter, Rejection, Reply};

use crate::{controllercolumn, with_db};

pub fn get_routes(
    db_pool: Pool<PgConnectionManager<NoTls>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_column = warp::path("column");
    db_column
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_db(db_pool.clone()))
        .and_then(controllercolumn::get_by_id)
        .or(db_column
            .and(warp::get())
            .and(with_db(db_pool.clone()))
            .and_then(controllercolumn::list_column_items_handler))
        .or(db_column
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(controllercolumn::create_column_items_handler))
        .or(db_column
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(controllercolumn::update_column_items_handler))
        .or(db_column
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db_pool.clone()))
            .and_then(controllercolumn::delete_column_items_handler))
}
