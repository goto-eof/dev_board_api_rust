use warp::{Filter, Rejection, Reply};

use crate::{controller::controller_column, util::util_authentication::auth_validator};
pub async fn get_column_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_column = warp::path("column");
    db_column
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(auth_validator("get_column".to_string()).await)
        .and_then(controller_column::get_column)
        .or(db_column
            .and(warp::get())
            .and(warp::path("all"))
            .and(warp::path::end())
            .and(auth_validator("get_all_columns".to_string()).await)
            .and_then(controller_column::get_all_columns))
        .or(db_column
            .and(warp::get())
            .and(warp::path("plus-items"))
            .and(warp::path::end())
            .and(auth_validator("get_all_columns_with_items".to_string()).await)
            .and_then(controller_column::get_all_columns_with_items))
        .or(db_column
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and(auth_validator("insert_column".to_string()).await)
            .and_then(controller_column::insert_column))
        .or(db_column
            .and(warp::put())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and(auth_validator("update_column".to_string()).await)
            .and_then(controller_column::update_column))
        .or(db_column
            .and(warp::put())
            .and(warp::path("swap"))
            .and(warp::path::end())
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and(auth_validator("swap_columns".to_string()).await)
            .and_then(controller_column::swap_columns))
        .or(db_column
            .and(warp::delete())
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(auth_validator("delete_column".to_string()).await)
            .and_then(controller_column::delete_column))
}

// pub fn extract_request(
// ) -> impl Filter<Extract = (http::Request<impl Stream>,), Error = warp::Rejection> + Copy {
//     warp::method()
//         .and(warp::path::full())
//         .and(warp::header::headers_cloned())
//         .and(warp::body::stream())
//         .map(
//             |method: http::Method, path: warp::path::FullPath, headers: http::HeaderMap, body| {
//                 let mut req = http::Request::builder()
//                     .method(method)
//                     .uri(path.as_str())
//                     .body(body)
//                     .expect("request builder");
//                 {
//                     *req.headers_mut() = headers;
//                 }
//                 req
//             },
//         )
// }

// pub fn http_request() -> impl Filter<Extract = (http::Request<JsonValue>,), Error = Rejection> + Copy
// {
//     // TODO: extract `hyper::Request` instead
//     // blocked by https://github.com/seanmonstar/warp/issues/139
//     warp::any()
//         .and(warp::method())
//         .and(warp::filters::path::full())
//         .and(warp::filters::query::raw())
//         .and(warp::header::headers_cloned())
//         .and(warp::body::bytes())
//         .and_then(|method, path: FullPath, query, headers, bytes| async move {
//             let uri = http::uri::Builder::new()
//                 .path_and_query(format!("{}?{}", path.as_str(), query))
//                 .build()
//                 .unwrap();

//             let mut request = http::Request::builder()
//                 .method(method)
//                 .uri(uri)
//                 .body(bytes)
//                 .unwrap();

//             *request.headers_mut() = headers;
//             *request.body_mut() = JsonValue::from(request.body().clone());

//             Ok::<http::Request<JsonValue>, Rejection>(request)
//         })
// }
