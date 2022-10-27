use serde::Serialize;
use warp::{reply::json, Reply};

use crate::Structs::{DaoError, Response};

pub fn generate_response<T: Serialize>(
    data: Result<T, DaoError>,
) -> crate::GenericResult<impl Reply> {
    match data {
        Ok(result) => Ok(json::<_>(&Response { result: &result })),
        Err(err) => Ok(json::<_>(&err)),
    }
}
