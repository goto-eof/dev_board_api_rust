use bcrypt::hash;
use chrono::Utc;
use entity::{db_permission, db_role, db_role_permission, db_user, db_user_role};
use migration::DbErr;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};

use crate::DB_POOL;

pub async fn init_admin() -> () {
    let db = DB_POOL.get().await;
    let result = db
        .transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // inserting user
                let user_admin_result = db_user::Entity::find()
                    .filter(db_user::Column::Username.eq("admin"))
                    .one(txn)
                    .await;
                if user_admin_result.is_err() {
                    return Ok(());
                }

                let user_admin_opt = user_admin_result.unwrap();

                if user_admin_opt.is_none() {
                    let data = r#"
                    {
                        "username": "admin",
                        "email": "admin@admin.com",
                        "first_name": "Admin",
                        "last_name": "Admin"
                    }"#;

                    let json_data = serde_json::from_str(data).unwrap();
                    let mut result_am = db_user::ActiveModel::from_json(json_data).unwrap();

                    let dat = Utc::now().naive_utc();
                    result_am.created_at = sea_orm::Set(Some(dat));
                    result_am.updated_at = sea_orm::Set(Some(dat));
                    result_am.password = Set(hash("password".to_string(), 4).unwrap());
                    let result = result_am.save(txn).await.unwrap();
                    let user_id = result.id.unwrap();

                    // inserting role
                    let mut result_am = db_role::ActiveModel::new();
                    result_am.name = sea_orm::Set("admin".to_string());
                    result_am.created_at = sea_orm::Set(Some(dat));
                    result_am.updated_at = sea_orm::Set(Some(dat));
                    let result = result_am.save(txn).await.unwrap();
                    let role_id = result.id.unwrap();

                    // associating role to user
                    let mut result_am = db_user_role::ActiveModel::new();
                    result_am.created_at = sea_orm::Set(Some(dat));
                    result_am.updated_at = sea_orm::Set(Some(dat));
                    result_am.role_id = Set(role_id);
                    result_am.user_id = Set(user_id);
                    let result = result_am.save(txn).await.unwrap();

                    let permissions = db_permission::Entity::find().all(txn).await.unwrap();
                    for permission in permissions {
                        let mut am = db_role_permission::ActiveModel::new();
                        am.created_at = sea_orm::Set(Some(dat));
                        am.updated_at = sea_orm::Set(Some(dat));
                        am.role_id = Set(role_id);
                        am.permission_id = Set(permission.id);

                        am.save(txn).await.unwrap();
                    }
                    return Ok(());
                }
                return Ok(());
            })
        })
        .await;
    if result.is_err() {
        println!("{:?}", result);
    }
}
