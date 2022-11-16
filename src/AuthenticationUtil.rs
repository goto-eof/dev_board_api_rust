use crate::{DB_POOL};
use chrono::Utc;
use entity::{db_user_role, db_role_permission, db_permission};
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use migration::JoinType;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QuerySelect};
use serde::{Deserialize, Serialize};
use warp::{self, reject, Filter, Rejection};
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: i32,
    exp: usize,
}
const JWT_SECRET: &str = "ciao mondo";

pub async fn auth_validator(
    permission_name: String,
) -> impl Filter<Extract = ((),), Error = Rejection> + Clone {
    let permission_name = warp::any().map(move || permission_name.clone());

    return warp::cookie::optional::<String>("Authorization")
        .and(warp::header::optional::<String>("Authorization"))
        .and(permission_name)
        .and_then(
            move |token: Option<String>, authorization: Option<String>, permission_name| async move {
                let tokeen = token.unwrap_or(authorization.unwrap_or("".to_string()));
                println!("token: {}", tokeen);
                let decoded = decode::<Claims>(
                    &tokeen,
                    &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
                    &Validation::new(jsonwebtoken::Algorithm::HS512),
                );
                let db = DB_POOL.get().await;

                if decoded.is_err() {
                    return Err(reject::custom(Unauthorized{error_message: "Invalid token".to_string()}));
                }
                let user_id = decoded.unwrap().claims.sub; 
               
            println!("___USERID: {}", user_id);

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


                println!("___UP__{:?}", user_permissions);
            
               
               
            //     let user_role_result = db_user_role::Entity::find().filter(db_user_role::Column::UserId.eq(userId))
            //         .one(db)
            //         .await;
            //     let role_id = user_role_result.unwrap().unwrap().role_id;

            //  let permissions_result = db_role_permission::Entity::find().filter(db_role_permission::Column::RoleId.eq(role_id)).all(db).await;
            // if permissions_result.is_err(){
            //         return  Err(warp::reject::custom(Unauthorized{error_message: "User not found".to_string()}));
            // }
            // let permissions = permissions_result.unwrap();
for user_permission in user_permissions {
    println!("___UP__{:?} {:?}", user_permission.name , permission_name);
if permission_name == user_permission.name   {
    return Ok(())
}
}
return  Err(warp::reject::custom(Unauthorized{error_message: "Permission not found".to_string()}));},
        );
}

pub fn generate_jwt(user_id: i32) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60)).unwrap()
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}

#[derive(Debug)]
struct Unauthorized{
    error_message: String
}
impl reject::Reject for Unauthorized {}
