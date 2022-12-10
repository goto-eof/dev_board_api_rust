use crate::{
    structure::structure::{DevBoardGenericError, Response},
    util::util_authentication::{generate_jwt, Claims},
    SETTINGS,
};
use chrono::{NaiveDateTime, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use warp::{
    self,
    http::HeaderValue,
    hyper::{HeaderMap, StatusCode},
    reply::json,
    Reply,
};

pub fn generate_response<T: Serialize>(
    data_wrapped: Result<T, DevBoardGenericError>,
    jwt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    let response = match data_wrapped {
        Ok(result) => json::<_>(&Response {
            success: true,
            result: &result,
        }),
        Err(err) => json::<_>(&err),
    };
    let reply = warp::reply::with_status(response, StatusCode::OK);

    if jwt.is_none() {
        return Ok(reply.into_response());
    }

    let old_jwt = jwt.unwrap();

    let decoded = decode::<Claims>(
        &old_jwt,
        &DecodingKey::from_secret(SETTINGS.jwt_secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    );
    let decoded = decoded.unwrap();
    let user_id = decoded.claims.sub;
    let exp = decoded.claims.exp;

    let new_jwt = calculate_jwt(exp, user_id, old_jwt);

    let mut response = reply.into_response();
    let mut cookies = HeaderMap::new();
    let cookie_str = format!(
        "token={}; SameSite=None; expires=Fri, 31 Dec 9999 23:59:59 GMT; Path=/; Secure; HttpOnly;",
        new_jwt
    );
    cookies.append(
        "set-cookie",
        HeaderValue::from_str(cookie_str.as_str()).unwrap(),
    );
    let headers = response.headers_mut();
    headers.extend(cookies);
    Ok(response)
}

fn calculate_jwt(exp: usize, user_id: i32, old_jwt: String) -> String {
    let datetime = NaiveDateTime::from_timestamp(exp.try_into().unwrap(), 0);
    let now = Utc::now().naive_local();
    let diff = datetime - now;
    let minutes = diff.num_minutes();
    println!(
        "Token info: is_expiring:{:?}, token_datetime: {:?}, now: {:?}, minutes: {}",
        minutes == 2,
        datetime,
        now,
        minutes
    );
    let new_jwt = match minutes {
        1 => {
            println!("\n\nToken refreshed\n\n");
            generate_jwt(user_id).unwrap()
        }
        _ => old_jwt,
    };
    new_jwt
}
