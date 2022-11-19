use serde_json::json;
use warp::{hyper::StatusCode, reply, Rejection, Reply};

use crate::{
    structure::structures::{DevBoardErrorType, DevBoardGenericError},
    util::util_authentication::Unauthorized,
};

pub(crate) async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if let Some(e) = err.find::<Unauthorized>() {
        let generic_error = DevBoardGenericError {
            success: false,
            message: e.error_message.to_owned(),
            code: -1,
            err_type: DevBoardErrorType::Error,
        };
        let resp = json!(generic_error);
        let res: String = resp.to_string();
        let boxed = Box::leak(res.into_boxed_str());
        Ok(reply::with_status(boxed, StatusCode::UNAUTHORIZED))
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
