use crate::ConfigurationLoader::Settings;
use async_once::AsyncOnce;
use log::debug;
use sea_orm::DbConn;
use warp::http::HeaderValue;
use warp::hyper::HeaderMap;
use warp::hyper::Method;
use warp::Filter;
use warp::Rejection;
use warp::Reply;
use RoutesColumn::get_column_routes;
use RoutesItem::get_item_routes;
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
    init_logging();
    init_db().await;
    init_server().await;
}

fn init_logging() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}

async fn init_db() {
    DB_POOL.get().await;
}

fn init_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
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
        .or(get_item_routes())
        .or(warp::options().map(warp::reply))
        .with(any_origin_3)
        .with(warp::log("api"))
        .with(warp::reply::with::headers(headers))
}

async fn init_server() {
    debug!("server run on port {}", SETTINGS.server_port);
    warp::serve(init_routes())
        .run(([0, 0, 0, 0], SETTINGS.server_port))
        .await;
}
