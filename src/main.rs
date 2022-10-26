use async_once::AsyncOnce;
use dotenv::dotenv;
use log::debug;
use sea_orm::ConnectOptions;
use std::time::Duration;
use warp::{Filter, Rejection};
use RoutesColumn::get_routes;

#[allow(non_snake_case)]
mod ControllerColumn;
#[allow(non_snake_case)]
mod DaoColumn;
#[allow(non_snake_case)]
mod RoutesColumn;

use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};

type GenericResult<T> = std::result::Result<T, Rejection>;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DB_POOOOOOL: AsyncOnce<DbConn> = AsyncOnce::new(async {
        let db = establish_connection().await;
        db.unwrap()
    });
}

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    //let database_url = std::env::var("DATABASE_URL").unwrap();
    let database_url = "postgres://postgres:postgres@127.0.0.1:5432/postgres".to_string();
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);
    let db = Database::connect(opt).await?;
    Migrator::up(&db, None).await?;
    debug!("DB Connection OK");
    Ok(db)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let hello = warp::path("hi").map(|| "Hello, World!");
    let routes = hello.or(get_routes()).with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
