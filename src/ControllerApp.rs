use crate::{DBPool, DatabaseConfig, ErrorManager::Error::*};
use warp::{http::StatusCode, reject, Reply};

pub async fn health_handler() -> crate::GenericResult<impl Reply> {
    // let db = DatabaseConfig::get_db_con(&db_pool)
    //     .await
    //     .map_err(|e| reject::custom(e))?;
    // db.execute("SELECT 1", &[])
    //     .await
    //     .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}
