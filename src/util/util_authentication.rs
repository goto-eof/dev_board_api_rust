use crate::{DB_POOL, SETTINGS};
use chrono::Utc;
use entity::{db_user_role, db_role_permission, db_permission};
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use log::debug;
use migration::JoinType;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QuerySelect};
use serde::{Deserialize, Serialize};
use warp::{self, reject, Filter, Rejection};
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

pub async fn auth_validator(
    permission_name: String
) -> impl Filter<Extract = (Option<String>,), Error = Rejection> + Clone {
    let permission_name = warp::any().map(move || permission_name.clone());

    return warp::cookie::optional::<String>("token")
        .and(warp::header::optional::<String>("Authorization"))
        .and(permission_name)
        .and_then(
            move |token: Option<String>, authorization: Option<String>, permission_name: String| async move {
                let tokeen = token.unwrap_or(authorization.unwrap_or("".to_string()));
                let decoded = decode::<Claims>(
                    &tokeen,
                    &DecodingKey::from_secret(SETTINGS.jwt_secret.as_bytes()),
                    &Validation::new(jsonwebtoken::Algorithm::HS256),
                );
                let db = DB_POOL.get().await;

                if decoded.is_err() {
                    return Err(reject::custom(Unauthorized{error_message: "Invalid token".to_string()}));
                }
                let decoded = decoded.unwrap();
                let user_id = decoded.claims.sub; 
               let exp = decoded.claims.exp;
               let now = Utc::now().timestamp() as usize;
               if exp < now{
                return Err(reject::custom(Unauthorized{error_message: "Token expired".to_string()}));
               }
             let user_permissions= db_permission::Entity::find()
               .join_rev(
                JoinType::InnerJoin,
                db_role_permission::Entity::belongs_to(db_permission::Entity)
                    .from(db_role_permission::Column::PermissionId)
                    .to(db_permission::Column::Id)
                    .into()
                )
                .join_rev(
                    JoinType::InnerJoin,
                    db_user_role::Entity::belongs_to(db_role_permission::Entity)
                        .from(db_user_role::Column::RoleId)
                        .to(db_role_permission::Column::RoleId)
                        .into()
                    ).filter(db_user_role::Column::UserId.eq(user_id))
                .all(db).await.unwrap();

                debug!("User permissions: {:?}", user_permissions);

for user_permission in user_permissions {
    if permission_name.eq(&user_permission.name)   {
        debug!("Permission found: {:?}", permission_name);
        return Ok(Some(tokeen))
    }
}
return  Err(warp::reject::custom(Unauthorized{error_message: "You have not permission to access to this resource".to_string()}));
},
        );
}

pub fn generate_jwt(user_id: i32) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(SETTINGS.jwt_ttl)).unwrap()
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS256);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(SETTINGS.jwt_secret.as_bytes()),
    )
}

pub fn extract_user_id(jwt_opt:  Option<String>)-> Option<i32>{
if jwt_opt.is_some(){
    let decoded = decode::<Claims>(
        &jwt_opt.unwrap(),
        &DecodingKey::from_secret(SETTINGS.jwt_secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    );
    return Some(decoded.unwrap().claims.sub);
}
return None;
}

#[derive(Debug, Serialize)]
pub struct Unauthorized{
    #[allow(dead_code)]// because is converted to json and sent to the fe 
    pub error_message: String
}
impl reject::Reject for Unauthorized {}