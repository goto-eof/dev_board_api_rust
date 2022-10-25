use dotenv::dotenv;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{Filter, Rejection};
use RoutesColumn::get_routes;

mod ControllerApp;
mod ControllerColumn;
mod DaoColumn;
mod DatabaseConfig;
mod ErrorManager;
mod RoutesColumn;
mod StructColumns;

type GenericResult<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let db_pool = DatabaseConfig::create_pool().unwrap();

    let db = DatabaseConfig::init_db(&db_pool).await;
    println!("{:?}", db);

    let health_route = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(ControllerApp::health_handler);
    let routes = health_route
        .or(get_routes(db_pool))
        .with(warp::cors().allow_any_origin())
        .recover(ErrorManager::handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
