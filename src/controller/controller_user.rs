use crate::{
    dao::dao_user, structure::structure::DevBoardGenericError, util::util_authentication::Claims,
    SETTINGS,
};

use super::controller_common;
use entity::db_user;
use jsonwebtoken::{decode, DecodingKey, Validation};
use warp::Reply;

pub async fn get_user_by_id(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_user::get_by_id(id).await, jwt_opt)
}

pub async fn get_user(jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    if jwt_opt.is_some() {
        let jwt = jwt_opt.unwrap();
        let decoded = decode::<Claims>(
            &jwt,
            &DecodingKey::from_secret(SETTINGS.jwt_secret.as_bytes()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        );
        let user_id = decoded.unwrap().claims.sub;
        let user = dao_user::get_by_id(user_id).await;
        let user = user.unwrap();
        let user = db_user::Model {
            created_at: user.created_at,
            email: user.email,
            first_name: user.first_name,
            id: -1,
            last_name: user.last_name,
            password: "".to_owned(),
            updated_at: user.updated_at,
            username: user.username,
        };
        return controller_common::generate_response(Ok(user), Some(jwt));
    }
    return controller_common::generate_response(
        Err(DevBoardGenericError {
            code: 0,
            err_type: crate::structure::structure::DevBoardErrorType::Error,
            message: "Invalid token".to_owned(),
            success: false,
        }),
        jwt_opt,
    );
}

pub async fn get_by_username(
    name: String,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_user::get_by_username(name).await, jwt_opt)
}

pub async fn get_all_users(jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_user::get_all().await, jwt_opt)
}

pub async fn get_all_users_for_sharing(
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(
        dao_user::get_all_for_sharing(jwt_opt.clone()).await,
        jwt_opt,
    )
}

pub async fn insert_user(
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_user::create(json_data).await, jwt_opt)
}

pub async fn update_user(
    id: i32,
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_user::update(id, json_data).await, jwt_opt)
}

pub async fn delete_user(id: i32, jwt_opt: Option<String>) -> crate::GenericResult<impl Reply> {
    controller_common::generate_response(dao_user::delete(id).await, jwt_opt)
}
