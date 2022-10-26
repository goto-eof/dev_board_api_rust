use async_once::AsyncOnce;
use dotenv::dotenv;
use entity::post;
use log::debug;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use sea_orm::{ActiveModelTrait, ConnectOptions, Set};
use std::{convert::Infallible, time::Duration};
use tokio_postgres::NoTls;
use warp::{Filter, Rejection};
use RoutesColumn::get_routes;

#[allow(non_snake_case)]
mod ControllerApp;
#[allow(non_snake_case)]
mod ControllerColumn;
#[allow(non_snake_case)]
mod DaoColumn;
#[allow(non_snake_case)]
mod DatabaseConfig;
#[allow(non_snake_case)]
mod ErrorManager;
#[allow(non_snake_case)]
mod MapperColumn;
#[allow(non_snake_case)]
mod RoutesColumn;
#[allow(non_snake_case)]
mod StructColumns;
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};

type GenericResult<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

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
    // .expect("Failed to setup the database");
    Migrator::up(&db, None).await?;
    // .expect("Failed to run migrations for tests");
    debug!("DB Connection OK");
    Ok(db)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    // let post = post::ActiveModel {
    //     title: Set(String::from("Amazing title 1")),
    //     text: Set(String::from("Lorem ipsum dolor sit amet.")),
    //     ..Default::default()
    // };
    // let post: post::Model = post.insert(&db).await.unwrap();
    // println!("Post created with ID: {}, TITLE: {}", post.id, post.title);

    let db_pool = DatabaseConfig::create_pool().unwrap();

    let db = DatabaseConfig::init_db(&db_pool).await;
    println!("{:?}", db);

    let health_route = warp::path!("health")
        // .and(with_db(db_pool.clone()))
        .and_then(ControllerApp::health_handler);
    let routes = health_route
        .or(get_routes())
        .with(warp::cors().allow_any_origin())
        .recover(ErrorManager::handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
