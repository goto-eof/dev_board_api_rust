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
use warp::Reply;

use crate::{AuthenticationUtil, ControllerCommon, Structs::DaoError, DB_POOL};

pub async fn login(login_data: LoginData) -> crate::GenericResult<impl Reply> {
    let db = DB_POOL.get().await;
    let user = db_user::Entity::find()
        .filter(db_user::Column::Username.eq(login_data.username))
        .one(db)
        .await;
    let user = user.unwrap();
    if user.is_none() {
        return ControllerCommon::generate_response(Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", "Invalid username"),
        }));
    }
    let user = user.unwrap();
    let check_password = verify(login_data.password, &user.password).unwrap();
    if check_password {
        return ControllerCommon::generate_response(Ok(JwtReponse {
            jwt: AuthenticationUtil::generate_jwt(user.id).unwrap(),
        }));
    }
    return ControllerCommon::generate_response(Err(DaoError {
        code: 1,
        err_type: crate::Structs::DaoErrorType::Error,
        message: format!("DB Error: {:?}", "Invalid password"),
    }));
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
