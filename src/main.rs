use async_once::AsyncOnce;
use dotenv::dotenv;
use sea_orm::DbConn;
use warp::{Filter, Rejection, Reply};
use RoutesColumn::get_column_routes;
use RoutesItem::get_item_routes;

use crate::ConfigurationLoader::Settings;

#[allow(non_snake_case)]
mod ConfigurationDatabase;
#[allow(non_snake_case)]
mod ConfigurationLoader;
#[allow(non_snake_case)]
mod ControllerColumn;
#[allow(non_snake_case)]
mod ControllerCommon;
#[allow(non_snake_case)]
mod ControllerItem;
#[allow(non_snake_case)]
mod DaoColumn;
#[allow(non_snake_case)]
mod DaoItem;
#[allow(non_snake_case)]
mod RoutesColumn;
#[allow(non_snake_case)]
mod RoutesItem;
#[allow(non_snake_case)]
mod Structs;

type GenericResult<T> = std::result::Result<T, Rejection>;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SETTINGS: Settings = Settings::init_configuration().unwrap();
    static ref DB_POOL: AsyncOnce<DbConn> = AsyncOnce::new(async {
        let db = ConfigurationDatabase::establish_connection().await;
        db.unwrap()
    });
}

#[tokio::main]
async fn main() {
    init_env();
    init_logging();
    Settings::init_configuration().unwrap();
    init_db().await;
    init_server().await;
}

fn init_env() {
    dotenv().ok();
}

fn init_logging() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}

async fn init_db() {
    DB_POOL.get().await;
}

fn init_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_column_routes()
        .or(get_item_routes())
        .with(warp::cors().allow_any_origin())
}

async fn init_server() {
    warp::serve(init_routes())
        .run(([0, 0, 0, 0], SETTINGS.server_port))
        .await;
}
