use chrono::Utc;
use entity::db_permission;
use log::debug;
use migration::DbErr;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter,
    TransactionTrait,
};

use crate::SETTINGS;

pub async fn init_permissions(db: &DbConn) {
    let result = db
        .transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                let dat = Utc::now().naive_utc();
                for permission in &SETTINGS.application_permissions {
                    let permission_model = db_permission::Entity::find()
                        .filter(db_permission::Column::Name.eq(permission as &str))
                        .one(txn)
                        .await;
                    if permission_model.is_ok() && permission_model.unwrap().is_none() {
                        let mut am = db_permission::ActiveModel::new();
                        am.name = sea_orm::Set(permission.to_string());
                        am.created_at = sea_orm::Set(Some(dat));
                        am.updated_at = sea_orm::Set(Some(dat));
                        let permission_result = am.save(txn).await.unwrap();
                        debug!("{:?}", permission_result)
                    }
                }
                return Ok(());
            })
        })
        .await;
    if result.is_err() {
        debug!("{:?}", result);
    }
}
