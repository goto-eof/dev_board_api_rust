use crate::{
    DBPool, DatabaseConfig, ErrorManager, ErrorManager::Error::*, MapperColumn::*, StructColumns::*,
};
type GenericResult<T> = std::result::Result<T, ErrorManager::Error>;

const TABLE: &str = "db_column_items";
const SELECT_FIELDS: &str = "ctm_id, ctm_name, created_at";

pub async fn get_by_id(db_pool: &DBPool, id: Option<i32>) -> GenericResult<DbColumnItems> {
    let con = DatabaseConfig::get_db_con(db_pool).await?;

    let query = format!(
        "SELECT {} FROM {} WHERE ctm_id = $1 ORDER BY created_at DESC",
        SELECT_FIELDS, TABLE
    );
    let q = con.query_one(query.as_str(), &[&id.unwrap()]).await;
    let row = q.map_err(DBQueryError)?;

    Ok(row_to_item(&row))
}

pub async fn fetch_all(db_pool: &DBPool) -> GenericResult<Vec<DbColumnItems>> {
    let con = DatabaseConfig::get_db_con(db_pool).await?;

    let query = format!(
        "SELECT {} FROM {} ORDER BY created_at DESC",
        SELECT_FIELDS, TABLE
    );
    let q = con.query(query.as_str(), &[]).await;

    let rows = q.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_item(&r)).collect())
}

pub async fn create(db_pool: &DBPool, body: DbColumnItemsRequest) -> GenericResult<DbColumnItems> {
    let con = DatabaseConfig::get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (ctm_name) VALUES ($1) RETURNING *", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.ctm_name])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_item(&row))
}

pub async fn update(
    db_pool: &DBPool,
    id: i32,

    body: DbColumnItemsUpdateRequest,
) -> GenericResult<DbColumnItems> {
    let con = DatabaseConfig::get_db_con(db_pool).await?;
    let query = format!("UPDATE {} SET ctm_name = $1 WHERE ctm_id = $2", TABLE);
    println!("{}", &query);
    con.execute(query.as_str(), &[&body.ctm_name, &id])
        .await
        .map_err(DBQueryError)?;

    let query = format!("SELECT * FROM {} WHERE ctm_id = $1", TABLE);
    println!("{}", &query);
    let row = con
        .query_one(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_item(&row))
}

pub async fn delete(db_pool: &DBPool, id: i32) -> GenericResult<u64> {
    let con = DatabaseConfig::get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE ctm_id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)
}
