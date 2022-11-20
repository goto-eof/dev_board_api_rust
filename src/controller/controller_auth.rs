#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}
use crate::structure::structure::DevBoardErrorType;
use crate::structure::structure::DevBoardGenericError;
use crate::structure::structure::LogoutResponse;
use crate::structure::structure::Response;
use crate::structure::structure::User;
use crate::util::util_authentication::{self};
use bcrypt::{hash, verify};
use chrono::Utc;
use entity::{db_role, db_user, db_user_role};
use migration::DbErr;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::{
    http::HeaderValue,
    hyper::{HeaderMap, StatusCode},
    Rejection, Reply,
};

use crate::DB_POOL;

pub async fn login(login_data: LoginData) -> Result<impl Reply, Rejection> {
    let db = DB_POOL.get().await;
    let user = db_user::Entity::find()
        .filter(db_user::Column::Username.eq(login_data.username))
        .one(db)
        .await;
    let user = user.unwrap();
    if user.is_none() {
        let err = DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", "Invalid username/password"),
        };
        let json = json!(err);
        return generate_response_with_cookie(json, None, StatusCode::BAD_REQUEST);
    }
    let user = user.unwrap();
    let check_password = verify(login_data.password, &user.password).unwrap();
    if check_password {
        let jwt = util_authentication::generate_jwt(user.id).unwrap();
        let user = User {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            username: user.username,
        };
        let response = Response {
            success: true,
            result: user,
        };
        let json = json!(response);
        return generate_response_with_cookie(json, Some(jwt), StatusCode::OK);
    }

    let err = DevBoardGenericError {
        success: false,
        code: 1,
        err_type: DevBoardErrorType::Error,
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

    let mut response = reply.into_response();

    if jwt.is_some() {
        let mut cookies = HeaderMap::new();
        let cookie_str = format!(
            "token={}; SameSite=None; expires=Fri, 31 Dec 9999 23:59:59 GMT; Path=/; Secure; HttpOnly;",
            jwt.unwrap()
        );
        cookies.append(
            "set-cookie",
            HeaderValue::from_str(cookie_str.as_str()).unwrap(),
        );
        let headers = response.headers_mut();
        headers.extend(cookies);
    }
    Ok(response)
}

pub async fn invalidate_token() -> Result<impl Reply, Rejection> {
    let logout_response = serde_json::json!(LogoutResponse { success: true });
    let reply = warp::reply::json(&logout_response);
    let reply = warp::reply::with_status(reply, StatusCode::OK);

    let mut response = reply.into_response();

    let mut cookies = HeaderMap::new();
    let cookie_str = format!(
        "token={}; SameSite=None; expires=Fri, 31 Dec 1999 23:59:59 GMT; Path=/; Secure; HttpOnly;",
        ""
    );
    cookies.append(
        "set-cookie",
        HeaderValue::from_str(cookie_str.as_str()).unwrap(),
    );
    let headers = response.headers_mut();
    headers.extend(cookies);
    Ok(response)
}

pub async fn register(registration_data: RegistrationData) -> Result<impl Reply, Rejection> {
    let db = DB_POOL.get().await;

    let result = db
        .transaction::<_, (Option<(i32, User)>, Option<String>), DbErr>(|txn| {
            let boxx = Box::pin(async move {
                let user = db_user::Entity::find()
                    .filter(
                        db_user::Column::Username
                            .eq(registration_data.username.clone())
                            .or(db_user::Column::Email.eq(registration_data.email.clone())),
                    )
                    .one(txn)
                    .await;
                if user.is_ok() {
                    let user = user.unwrap();
                    if user.is_some() {
                        return Ok((None, Some("User already exists".to_string())));
                    }
                }
                let hashed_password = hash(registration_data.password, 4).unwrap();
                let dat = Utc::now().naive_utc();
                let mut user_model = db_user::ActiveModel {
                    username: Set(registration_data.username),
                    password: Set(hashed_password),
                    email: Set(registration_data.email),
                    created_at: Set(Some(dat)),
                    updated_at: Set(Some(dat)),
                    ..Default::default()
                };

                if registration_data.first_name.is_some() {
                    user_model.first_name = Set(registration_data.first_name.unwrap());
                }
                if registration_data.last_name.is_some() {
                    user_model.last_name = Set(registration_data.last_name.unwrap());
                }
                let user_model = user_model.save(txn).await;
                let user_model = user_model.unwrap();

                // assigning role to user
                let mut ur_am = db_user_role::ActiveModel::new();
                ur_am.user_id = Set(user_model.id.clone().unwrap());
                ur_am.role_id = Set(db_role::Entity::find()
                    .filter(db_role::Column::Name.eq("user"))
                    .one(db)
                    .await
                    .unwrap()
                    .unwrap()
                    .id);
                let dat = Utc::now().naive_utc();
                ur_am.created_at = sea_orm::Set(Some(dat));
                ur_am.updated_at = sea_orm::Set(Some(dat));
                let user = ur_am.save(txn).await;
                if user.is_err() {
                    return Ok((None, Some("Error creating user".to_string())));
                }
                user.unwrap();
                let user = User {
                    first_name: user_model.first_name.unwrap(),
                    last_name: user_model.last_name.unwrap(),
                    email: user_model.email.unwrap(),
                    username: user_model.username.unwrap(),
                };
                return Ok((Some((user_model.id.unwrap(), user)), None));
            });
            boxx
        })
        .await;
    if result.is_ok() {
        let result = result.unwrap();
        if result.0.is_some() {
            let user = result.0.unwrap();
            let jwt = util_authentication::generate_jwt(user.0).unwrap();
            let response = Response {
                success: true,
                result: user.1,
            };
            let json = json!(response);
            return generate_response_with_cookie(json, Some(jwt), StatusCode::OK);
        } else {
            let err = DevBoardGenericError {
                success: false,
                code: 1,
                err_type: DevBoardErrorType::Error,
                message: format!("DB Error: {:?}", result.1.unwrap()),
            };
            let json = json!(err);
            return generate_response_with_cookie(json, None, StatusCode::BAD_REQUEST);
        }
    }

    let err = DevBoardGenericError {
        success: false,
        code: 1,
        err_type: DevBoardErrorType::Error,
        message: format!("DB Error: {:?}", result.err().unwrap()),
    };
    let json = json!(err);
    return generate_response_with_cookie(json, None, StatusCode::BAD_REQUEST);
}
