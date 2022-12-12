use crate::{
    structure::structure::{DevBoardGenericError, Message, Response},
    util::util_authentication::Claims,
    SETTINGS,
};
use chrono::{NaiveDateTime, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use warp::{self, hyper::StatusCode, reply::json, Reply};

pub fn generate_response<T: Serialize>(
    data_wrapped: Result<T, DevBoardGenericError>,
    jwt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    if jwt.is_none() {
        let response = json::<_>(&Response {
            success: true,
            result: Message {
                message: "Need authentication".to_owned(),
            },
            refresh_token: false,
        });
        let reply = warp::reply::with_status(response, StatusCode::UNAUTHORIZED);
        return Ok(reply.into_response());
    }

    let old_jwt = jwt.unwrap();
    let decoded = decode::<Claims>(
        &old_jwt,
        &DecodingKey::from_secret(SETTINGS.jwt_secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    );
    let decoded = decoded.unwrap();
    let exp = decoded.claims.exp;

    let response = match data_wrapped {
        Ok(result) => json::<_>(&Response {
            success: true,
            result: &result,
            refresh_token: is_token_to_refresh(exp),
        }),
        Err(err) => json::<_>(&err),
    };
    let reply = warp::reply::with_status(response, StatusCode::OK);

    let response = reply.into_response();
    Ok(response)
}

fn is_token_to_refresh(exp: usize) -> bool {
    let datetime = NaiveDateTime::from_timestamp(exp.try_into().unwrap(), 0);
    let now = Utc::now().naive_local();
    let diff = datetime - now;
    let minutes = diff.num_minutes();
    return minutes == 0;
}
