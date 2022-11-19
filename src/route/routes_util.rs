use super::{
    routes_column::get_column_routes, routes_item::get_item_routes,
    routes_permission::get_permission_routes, routes_role::get_role_routes,
    routes_user::get_user_routes,
};
use crate::{
    structure::structure::{DevBoardErrorType, DevBoardGenericError},
    util::util_authentication::Unauthorized,
};
use warp::{
    hyper::{Method, StatusCode},
    Filter, Rejection, Reply,
};

pub(crate) async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        return generate_response("Not found".to_string(), StatusCode::NOT_FOUND);
    } else if let Some(e) = err.find::<Unauthorized>() {
        let message = e.error_message.to_owned();
        return generate_response(message, StatusCode::UNAUTHORIZED);
    } else {
        return generate_response(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
    }
}

fn generate_response(
    message: String,
    status_code: StatusCode,
) -> Result<warp::hyper::Response<warp::hyper::Body>, Rejection> {
    let generic_error = DevBoardGenericError {
        success: false,
        message,
        code: 0,
        err_type: DevBoardErrorType::Error,
    };
    let json = warp::reply::json(&generic_error);
    let reply_message = warp::reply::with_status(json, status_code);
    Ok(reply_message.into_response())
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
