use async_once::AsyncOnce;
use dotenv::dotenv;
use warp::{Filter, Rejection};
use RoutesColumn::get_routes;

#[allow(non_snake_case)]
mod ConfigurationDatabase;
#[allow(non_snake_case)]
mod ControllerColumn;
#[allow(non_snake_case)]
mod DaoColumn;
#[allow(non_snake_case)]
mod RoutesColumn;

use sea_orm::DbConn;

type GenericResult<T> = std::result::Result<T, Rejection>;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DB_POOL: AsyncOnce<DbConn> = AsyncOnce::new(async {
        let db = ConfigurationDatabase::establish_connection().await;
        db.unwrap()
    });
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let hello = warp::path("hi").map(|| "Hello, World!");
    let routes = hello.or(get_routes()).with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
