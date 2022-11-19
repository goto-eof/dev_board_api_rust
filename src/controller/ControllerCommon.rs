use crate::structure::Structures::DevBoardGenericError;
use crate::structure::Structures::Response;
use serde::Serialize;
use warp::{reply::json, Reply};
pub fn generate_response<T: Serialize>(
    data: Result<T, DevBoardGenericError>,
) -> crate::GenericResult<impl Reply> {
    match data {
        Ok(result) => Ok(json::<_>(&Response {
            success: true,
            result: &result,
        })),
        Err(err) => Ok(json::<_>(&err)),
    }
}
