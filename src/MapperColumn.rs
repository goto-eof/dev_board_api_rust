use chrono::prelude::*;
use mobc_postgres::tokio_postgres;
use tokio_postgres::Row;

use crate::StructColumns::DbColumnItems;

pub fn row_to_item(row: &Row) -> DbColumnItems {
    let ctm_id: i32 = row.get(0);
    let ctm_name: String = row.get(1);
    let created_at: DateTime<Utc> = row.get(2);
    DbColumnItems {
        ctm_id,
        ctm_name,
        created_at,
    }
}
