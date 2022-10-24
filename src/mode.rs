use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DbColumnItems {
    pub ctm_id: i32,
    pub ctm_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct DbColumnItemsRequest {
    pub ctm_name: String,
}

#[derive(Deserialize)]
pub struct DbColumnItemsUpdateRequest {
    pub ctm_name: String,
}

#[derive(Serialize)]
pub struct DbColumnItemsResponse {
    pub ctm_id: i32,
    pub ctm_name: String,
}

impl DbColumnItemsResponse {
    pub fn of(todo: DbColumnItems) -> DbColumnItemsResponse {
        DbColumnItemsResponse {
            ctm_id: todo.ctm_id,
            ctm_name: todo.ctm_name,
        }
    }
}
