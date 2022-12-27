use super::{
    routes_attachment::get_attachment_routes, routes_board::get_board_routes,
    routes_column::get_column_routes, routes_item::get_item_routes,
    routes_message::get_message_routes, routes_permission::get_permission_routes,
    routes_role::get_role_routes, routes_user::get_user_routes,
};
use crate::{
    structure::structure::{DevBoardErrorType, DevBoardGenericError},
    util::util_authentication::Unauthorized,
    SETTINGS,
};
use warp::{hyper::StatusCode, Filter, Rejection, Reply};

pub(crate) async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        return generate_response("Not found".to_string(), StatusCode::NOT_FOUND, 8563);
    } else if let Some(e) = err.find::<Unauthorized>() {
        let message = e.error_message.to_owned();
        return generate_response(message, StatusCode::UNAUTHORIZED, 1579);
    } else {
        return generate_response(
            "Internal Server Error: path not found?".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
            3748,
        );
    }
}

fn generate_response(
    message: String,
    status_code: StatusCode,
    code: i32,
) -> Result<warp::hyper::Response<warp::hyper::Body>, Rejection> {
    let generic_error = DevBoardGenericError {
        success: false,
        message,
        code,
        err_type: DevBoardErrorType::Error,
    };
    let json = warp::reply::json(&generic_error);
    let reply_message = warp::reply::with_status(json, status_code);
    Ok(reply_message.into_response())
}

pub async fn init_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // let cors_allowed_origins: Vec<&str> = SETTINGS
    //     .cors_allowed_origins
    //     .iter()
    //     .map(|s| s as &str)
    //     .collect();

    let cors_allowed_headers: Vec<&str> = SETTINGS
        .cors_allowed_headers
        .iter()
        .map(|s| s as &str)
        .collect();

    let cors_allowed_methods: Vec<&str> = SETTINGS
        .cors_allowed_methods
        .iter()
        .map(|s| s as &str)
        .collect();

    let any_origin_3 = warp::cors()
        // .allow_origins(cors_allowed_origins)
        .allow_any_origin()
        .allow_headers(cors_allowed_headers)
        .allow_methods(cors_allowed_methods)
        .allow_credentials(true);

    get_column_routes()
        .await
        .or(get_item_routes().await)
        .or(get_user_routes().await)
        .or(get_role_routes().await)
        .or(get_permission_routes().await)
        .or(get_board_routes().await)
        .or(get_message_routes().await)
        .or(get_attachment_routes().await)
        .recover(handle_rejection)
        .with(&any_origin_3)
        .with(warp::log("api"))
}
