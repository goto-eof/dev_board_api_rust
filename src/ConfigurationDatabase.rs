use log::debug;
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::ConnectOptions;
use sea_orm::{Database, DbConn};

use crate::SETTINGS;

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let opt = ConnectOptions::new(SETTINGS.db_uri.to_owned());
    // opt.max_connections(100)
    //     .min_connections(5)
    //     .connect_timeout(Duration::from_secs(8))
    //     .idle_timeout(Duration::from_secs(8))
    //     .max_lifetime(Duration::from_secs(8))
    //     .sqlx_logging(true)
    //     .sqlx_logging_level(log::LevelFilter::Debug);
    let db = Database::connect(opt).await?;
    Migrator::up(&db, None).await?;
    debug!("DB Connection OK: {}", SETTINGS.db_uri);
    Ok(db)
}
