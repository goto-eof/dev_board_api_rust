use crate::configuration::config_database;
use crate::configuration::config_loader::Settings;
use crate::route::routes_util::init_routes;
use async_once::AsyncOnce;
use configuration::config_database::init_db;
use configuration::config_logging::init_logging;
use dao::dao_common;
use log::debug;
use sea_orm::DbConn;
use util::util_permission::init_permissions;
use warp::Rejection;
mod configuration;
mod controller;
mod dao;
mod route;
mod structure;
mod util;
type GenericResult<T> = std::result::Result<T, Rejection>;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SETTINGS: Settings = Settings::init_configuration().unwrap();
    static ref DB_POOL: AsyncOnce<DbConn> = AsyncOnce::new(async {
        let db = config_database::establish_connection().await;
        db.unwrap()
    });
}

#[tokio::main]
async fn main() {
    init_logging();
    init_db().await;
    init_permissions(DB_POOL.get().await).await;
    dao_common::init_admin().await; // default superuser
    dao_common::init_user_role().await; // this role is assigned when a new user is created
    init_server().await;
}

async fn init_server() {
    debug!("server run on port {}", SETTINGS.server_port);
    warp::serve(init_routes().await)
        .run(([0, 0, 0, 0], SETTINGS.server_port))
        .await;
}
