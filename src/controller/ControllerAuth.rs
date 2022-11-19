#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}
use crate::structure::Structures::DaoError;
use crate::structure::Structures::DaoErrorType;
use crate::util::AuthenticationUtil::{self};
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
        let err = DaoError {
            code: 1,
            err_type: DaoErrorType::Error,
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
        err_type: DaoErrorType::Error,
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

pub async fn register(registration_data: RegistrationData) -> Result<impl Reply, Rejection> {
    let db = DB_POOL.get().await;

    let result = db
        .transaction::<_, (Option<i32>, Option<String>), DbErr>(|txn| {
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
                let user = user.save(txn).await;
                let user = user.unwrap();

                let mut ur_am = db_user_role::ActiveModel::new();
                ur_am.user_id = Set(user.id.unwrap());
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
                let user = user.unwrap();
                println!("{:?}", user);
                return Ok((Some(user.id.unwrap()), None));
            });
            boxx
        })
        .await;
    if result.is_ok() {
        let result = result.unwrap();
        if result.0.is_some() {
            let user_id = result.0.unwrap();
            let jwt = AuthenticationUtil::generate_jwt(user_id).unwrap();
            let json = json!(user_id);
            return generate_response_with_cookie(json, Some(jwt), StatusCode::OK);
        } else {
            let err = DaoError {
                code: 1,
                err_type: DaoErrorType::Error,
                message: format!("DB Error: {:?}", result.1.unwrap()),
            };
            let json = json!(err);
            return generate_response_with_cookie(json, None, StatusCode::BAD_REQUEST);
        }
    }

    let err = DaoError {
        code: 1,
        err_type: DaoErrorType::Error,
        message: format!("DB Error: {:?}", result.err().unwrap()),
    };
    let json = json!(err);
    return generate_response_with_cookie(json, None, StatusCode::BAD_REQUEST);
}
