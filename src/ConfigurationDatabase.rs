use log::debug;
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::ConnectOptions;
use sea_orm::{Database, DbConn};

const DOCKER_DB_URI: &str = "postgres://postgres:postgres@postgres_service:5432/postgres";

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let database_uri_result = std::env::var("DATABASE_URL");
    let database_uri = match database_uri_result {
        Ok(value) => value,
        Err(_) => DOCKER_DB_URI.to_owned(),
    };
    let opt = ConnectOptions::new(database_uri.to_owned());
    // opt.max_connections(100)
    //     .min_connections(5)
    //     .connect_timeout(Duration::from_secs(8))
    //     .idle_timeout(Duration::from_secs(8))
    //     .max_lifetime(Duration::from_secs(8))
    //     .sqlx_logging(true)
    //     .sqlx_logging_level(log::LevelFilter::Debug);
    let db = Database::connect(opt).await?;
    Migrator::up(&db, None).await?;
    debug!("DB Connection OK: {}", database_uri);
    Ok(db)
}
