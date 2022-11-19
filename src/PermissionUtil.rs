use chrono::Utc;
use entity::db_permission;
use migration::DbErr;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter,
    TransactionTrait,
};

pub async fn init_permissions(db: &DbConn) {
    let result = db
        .transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                let permissions = vec![
                    "get_column",
                    "get_all_columns",
                    "get_all_columns_with_items",
                    "insert_column",
                    "update_column",
                    "swap_columns",
                    "delete_column",
                    "get_item",
                    "get_items",
                    "insert_item",
                    "update_item",
                    "delete_item",
                    "swap_items",
                ];
                let dat = Utc::now().naive_utc();
                for permission in permissions {
                    let permission_model = db_permission::Entity::find()
                        .filter(db_permission::Column::Name.eq(permission))
                        .one(txn)
                        .await;

                    if permission_model.is_ok() && permission_model.unwrap().is_none() {
                        let mut am = db_permission::ActiveModel::new();
                        am.name = sea_orm::Set(permission.to_string());
                        am.created_at = sea_orm::Set(Some(dat));
                        am.updated_at = sea_orm::Set(Some(dat));
                        let permission_result = am.save(txn).await.unwrap();
                        println!("{:?}", permission_result)
                    }
                }
                return Ok(());
            })
        })
        .await;
    if result.is_err() {
        println!("{:?}", result);
    }
}
