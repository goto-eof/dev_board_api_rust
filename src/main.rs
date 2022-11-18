use crate::ConfigurationLoader::Settings;
use ::function_name::named;
use async_once::AsyncOnce;
use log::debug;
use migration::DbErr;
use sea_orm::ConnectionTrait;
use sea_orm::DbConn;
use sea_orm::Statement;
use sea_orm::TransactionTrait;
use warp::http::HeaderValue;
use warp::hyper::HeaderMap;
use warp::hyper::Method;
use warp::Filter;
use warp::Rejection;
use warp::Reply;
use PermissionUtil::init_permissions;
use RoutesColumn::get_column_routes;
use RoutesItem::get_item_routes;
use RoutesUser::get_user_routes;
#[allow(non_snake_case)]
mod AuthenticationUtil;
#[allow(non_snake_case)]
mod ConfigurationDatabase;
#[allow(non_snake_case)]
mod ConfigurationLoader;
#[allow(non_snake_case)]
mod ControllerAuth;
#[allow(non_snake_case)]
mod ControllerColumn;
#[allow(non_snake_case)]
mod ControllerCommon;
#[allow(non_snake_case)]
mod ControllerItem;
#[allow(non_snake_case)]
mod ControllerUser;
#[allow(non_snake_case)]
mod DaoColumn;
#[allow(non_snake_case)]
mod DaoCommon;
#[allow(non_snake_case)]
mod DaoItem;
#[allow(non_snake_case)]
mod DaoPermission;
#[allow(non_snake_case)]
mod DaoRole;
#[allow(non_snake_case)]
mod DaoUser;
#[allow(non_snake_case)]
mod PermissionUtil;
#[allow(non_snake_case)]
mod RoutesColumn;
#[allow(non_snake_case)]
mod RoutesItem;
#[allow(non_snake_case)]
mod RoutesUser;
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
    init_logging();
    init_db().await;
    init_permissions(DB_POOL.get().await).await;
    DaoCommon::init_admin().await;
    DaoCommon::init_user_role().await;
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

async fn init_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let any_origin_3 = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Access-Control-Allow-Headers",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Origin",
            "Accept",
            "X-Requested-With",
            "Content-Type",
        ])
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
            Method::HEAD,
        ]);

    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("*"),
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("*"),
    );

    get_column_routes()
        .await
        .or(get_item_routes().await)
        .or(get_user_routes().await)
        .or(warp::options().map(warp::reply))
        .with(any_origin_3)
        .with(warp::log("api"))
        .with(warp::reply::with::headers(headers))
}

async fn init_server() {
    debug!("server run on port {}", SETTINGS.server_port);
    warp::serve(init_routes().await)
        .run(([0, 0, 0, 0], SETTINGS.server_port))
        .await;
}
