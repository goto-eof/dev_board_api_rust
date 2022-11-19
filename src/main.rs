use crate::configuration::config_database;
use crate::configuration::config_loader::Settings;
use crate::route::routes_util::init_routes;
use ::function_name::named;
use async_once::AsyncOnce;
use dao::dao_common;
use log::debug;
use sea_orm::ConnectionTrait;
use sea_orm::DbConn;
use sea_orm::Statement;
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
    init_test();
    init_server().await;
}

fn init_test() {}

fn init_logging() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}

#[named]
async fn init_db() {
    println!("FN: {:?}", function_name!());
    debug!("Checking DB connection...");
    let db = DB_POOL.get().await;
    let result = db
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT 1 from db_column limit 1;".to_owned(),
        ))
        .await;
    if result.is_err() {
        debug!("[DB RESULT] Connection to [DB FAILED]: {:?}", result.err());
    } else {
        debug!("[DB RESULT] DB Connection [OK]")
    }
}

async fn init_server() {
    debug!("server run on port {}", SETTINGS.server_port);
    warp::serve(init_routes().await)
        .run(([0, 0, 0, 0], SETTINGS.server_port))
        .await;
}
