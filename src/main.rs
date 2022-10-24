use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use route::get_routes;
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{Filter, Rejection};

mod dao_column;
mod dbconfig;
mod error;
mod handler_column;
mod model;
mod route;

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let db_pool = dbconfig::create_pool().unwrap();

    let db = dbconfig::init_db(&db_pool).await;
    println!("{:?}", db);

    let health_route = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(handler_column::health_handler);
    let routes = health_route
        .or(get_routes(db_pool))
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
