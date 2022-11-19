use serde_json::json;
use warp::{
    hyper::{Method, StatusCode},
    reply, Filter, Rejection, Reply,
};

use crate::{
    structure::structures::{DevBoardErrorType, DevBoardGenericError},
    util::util_authentication::Unauthorized,
};

use super::{
    routes_column::get_column_routes, routes_item::get_item_routes,
    routes_permission::get_permission_routes, routes_role::get_role_routes,
    routes_user::get_user_routes,
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

pub async fn init_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let any_origin_3 = warp::cors()
        // .allow_any_origin()
        .allow_origin("http://localhost:3000")
        .allow_headers(vec![
            "Access-Control-Allow-Credentials",
            "Access-Control-Allow-Headers",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Origin",
            "Accept",
            "Content-Type",
            "Accept-Encoding",
            "Accept-Language",
            "Cache-Control",
            "Connection",
            "Host",
            "Pragma",
            "Referer",
            "User-Agent",
            "X-Requested-With",
            "Content-Type",
            "Cookie",
            "sec-ch-ua",
            "sec-ch-ua-mobile",
            "sec-ch-ua-platform",
            "Sec-Fetch-Dest",
            "Sec-Fetch-Mode",
            "Sec-Fetch-Site",
            "Sec-Fetch-User",
            "Sec-WebSocket-Extensions",
            "Sec-WebSocket-Key",
            "Sec-WebSocket-Version",
            "Upgrade-Insecure-Requests",
            "Upgrade",
            "Authorization",
        ])
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
            Method::HEAD,
        ])
        .allow_credentials(true);

    get_column_routes()
        .await
        .or(get_item_routes().await)
        .or(get_user_routes().await)
        .or(get_role_routes().await)
        .or(get_permission_routes().await)
        .recover(handle_rejection)
        .with(&any_origin_3)
        .with(warp::log("api"))
}
