#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

use bcrypt::{hash, verify};
use chrono::Utc;
use entity::db_user;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::{
    http::HeaderValue,
    hyper::{header, HeaderMap, StatusCode},
    Rejection, Reply,
};

use crate::{AuthenticationUtil, ControllerCommon, Structs::DaoError, DB_POOL};

pub async fn login(login_data: LoginData) -> Result<impl Reply, Rejection> {
    let db = DB_POOL.get().await;
    let user = db_user::Entity::find()
        .filter(db_user::Column::Username.eq(login_data.username))
        .one(db)
        .await;
    let user = user.unwrap();
    if user.is_none() {
        let err = DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", "Invalid username/password"),
        };
        let json = json!(err);
        return generate_response_with_cookie(json, None, StatusCode::BAD_REQUEST);
    }
    let user = user.unwrap();
    let check_password = verify(login_data.password, &user.password).unwrap();
    if check_password {
        let jwt = AuthenticationUtil::generate_jwt(user.id).unwrap();
        let json = json!(user.email);
        return generate_response_with_cookie(json, Some(jwt), StatusCode::OK);
    }

    let err = DaoError {
        code: 1,
        err_type: crate::Structs::DaoErrorType::Error,
        message: format!("DB Error: {:?}", "Invalid username/password"),
    };
    let json = json!(err);
    return generate_response_with_cookie(json, None, StatusCode::BAD_REQUEST);
}
#[derive(Debug, Serialize)]
pub struct JwtReponse {
    pub jwt: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationData {
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

pub fn generate_response_with_cookie(
    response: serde_json::Value,
    jwt: Option<String>,
    status_code: StatusCode,
) -> Result<impl Reply, Rejection> {
    let reply = warp::reply::json(&response);
    let reply = warp::reply::with_status(reply, status_code);

    let mut cookies = HeaderMap::new();
    let cookie_str = format!("jwt={}; Path=/; HttpOnly; Max-Age=1209600", jwt.unwrap());
    cookies.append(
        header::SET_COOKIE,
        HeaderValue::from_str(cookie_str.as_str()).unwrap(),
    );
    let mut response = reply.into_response();
    let headers = response.headers_mut();
    headers.extend(cookies);
    Ok(response)
}

pub async fn register(registration_data: RegistrationData) -> crate::GenericResult<impl Reply> {
    let db = DB_POOL.get().await;

    let user = db_user::Entity::find()
        .filter(
            db_user::Column::Username
                .eq(registration_data.username.clone())
                .or(db_user::Column::Email.eq(registration_data.email.clone())),
        )
        .one(db)
        .await;
    if user.is_ok() {
        let user = user.unwrap();
        if user.is_some() {
            return ControllerCommon::generate_response(Err(DaoError {
                code: 1,
                err_type: crate::Structs::DaoErrorType::Error,
                message: format!("DB Error: {:?}", "User already exists"),
            }));
        }
    }
    let hashed_password = hash(registration_data.password, 4).unwrap();
    let dat = Utc::now().naive_utc();
    let mut user = db_user::ActiveModel {
        username: Set(registration_data.username),
        password: Set(hashed_password),
        email: Set(registration_data.email),
        created_at: Set(Some(dat)),
        updated_at: Set(Some(dat)),
        ..Default::default()
    };

    if registration_data.first_name.is_some() {
        user.first_name = Set(registration_data.first_name.unwrap());
    }
    if registration_data.last_name.is_some() {
        user.last_name = Set(registration_data.last_name.unwrap());
    }
    let user = user.save(db).await;

    // TODO assign permissions

    if user.is_err() {
        return ControllerCommon::generate_response(Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?} {:?}", "Can't create account", user),
        }));
    }

    let user = user.unwrap();

    println!("{:?}", user);
    return ControllerCommon::generate_response(Ok(JwtReponse {
        jwt: AuthenticationUtil::generate_jwt(user.id.unwrap()).unwrap(),
    }));
}
