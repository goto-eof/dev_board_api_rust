use std::result;

use crate::structure::structure::DevBoardErrorType;
use crate::structure::structure::DevBoardGenericError;
use crate::util::util_authentication::extract_user_id;
use crate::DB_POOL;
use base64::decode;
use chrono::Utc;
use entity::db_attachment;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::Set;

pub async fn get_by_id(id: i32) -> Result<db_attachment::Model, DevBoardGenericError> {
    let db = DB_POOL.get().await;
    let result = db_attachment::Entity::find_by_id(id).one(db).await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    Ok(opt.unwrap())
}

pub async fn get_by_item_id(
    parent_id: i32,
) -> Result<Vec<db_attachment::Model>, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let result = db_attachment::Entity::find()
        .filter(db_attachment::Column::ItemId.eq(parent_id))
        .order_by_desc(db_attachment::Column::Id)
        .all(db)
        .await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let models = result.unwrap();

    Ok(models)
}

pub async fn create(
    json_data: serde_json::Value,
) -> Result<db_attachment::Model, DevBoardGenericError> {
    let db = DB_POOL.get().await;
    let result = db_attachment::ActiveModel::from_json(json_data);

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let mut model = result.unwrap();

    let dat = Utc::now().naive_utc();

    model.created_at = sea_orm::Set(Some(dat));
    model.updated_at = sea_orm::Set(Some(dat));

    let result = model.insert(db).await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    Ok(result.unwrap())
}

pub async fn update(
    id: i32,
    json_data: serde_json::Value,
) -> Result<db_attachment::Model, DevBoardGenericError> {
    let db = DB_POOL.get().await;
    let result = db_attachment::Entity::find_by_id(id).one(db).await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let mut item_active_model: db_attachment::ActiveModel = opt.unwrap().into();

    let result = item_active_model.set_from_json(json_data);

    if result.is_err() {
        if result.is_err() {
            return Err(DevBoardGenericError {
                success: false,
                code: 1,
                err_type: DevBoardErrorType::Error,
                message: format!("DB Error: {:?}", result.err()),
            });
        }
    }

    let dat = Utc::now().naive_utc();
    item_active_model.updated_at = sea_orm::Set(Some(dat));

    let result = item_active_model.update(db).await;

    if result.is_err() {
        if result.is_err() {
            return Err(DevBoardGenericError {
                success: false,
                code: 1,
                err_type: DevBoardErrorType::Error,
                message: format!("DB Error: {:?}", result.err()),
            });
        }
    }

    Ok(result.unwrap())
}

pub async fn delete(id: i32, jwt_opt: Option<String>) -> Result<bool, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let user_id = extract_user_id(jwt_opt.clone());

    if user_id.is_none() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let user_id = user_id.unwrap();

    let result = db_attachment::Entity::find_by_id(id)
        .filter(db_attachment::Column::UserId.eq(user_id))
        .one(db)
        .await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let result = opt.unwrap().delete(db).await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let deletion_result = result.unwrap();
    assert_eq!(deletion_result.rows_affected, 1);
    Ok(deletion_result.rows_affected == 1)
}

pub async fn save_files(user_id: i32, item_id: i32, files: serde_json::Value) -> () {
    let db = DB_POOL.get().await;
    let files = files.as_array();
    let files = files.unwrap();
    for file in files {
        let name = file["name"].as_str().unwrap();
        let hashcode = file["hashcode"].as_str().unwrap();
        let content = file["content"].as_str().unwrap();
        let start_pos = content.chars().position(|c| c == ',').unwrap() + 1;
        let end_pos = content.chars().count();
        let decoded = &decode(&content[start_pos..end_pos]);
        let decoded = decoded.clone().unwrap();

        let file_name = format!("/Users/andrei/Desktop/{}.{}", hashcode, "png");
        tokio::fs::write(&file_name, decoded)
            .await
            .map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })
            .unwrap();

        let dat = Utc::now().naive_utc();

        let result_attachments = db_attachment::ActiveModel {
            created_at: Set(Some(dat)),
            updated_at: Set(Some(dat)),
            hashcode: Set(hashcode.to_owned()),
            name: Set(name.to_owned()),
            item_id: Set(item_id),
            user_id: Set(user_id),
            id: NotSet,
            ..Default::default()
        }
        .into_active_model()
        .insert(db)
        .await;
        if result_attachments.is_err() {
            println!("{:?}", result_attachments.err());
        }
    }
}