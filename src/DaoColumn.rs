use crate::DB_POOOOOOL;
use entity::db_column;
use sea_orm::ActiveModelTrait;
use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
use serde::Serialize;
#[derive(Serialize)]
pub enum DaoErrorType {
    Error,
    Warning,
}
#[derive(Serialize)]
pub struct DaoError {
    code: i32,
    err_type: DaoErrorType,
    message: String,
}

pub async fn get_by_id(id: i32) -> Result<db_column::Model, DaoError> {
    let db = DB_POOOOOOL.get().await;
    let item = db_column::Entity::find_by_id(id).one(db).await;

    if item.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: DaoErrorType::Error,
            message: format!("DB Error: {:?}", item.err()),
        });
    }

    let opt = item.unwrap();

    if opt.is_none() {
        return Err(DaoError {
            code: 2,
            err_type: DaoErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    Ok(opt.unwrap())
}

pub async fn get_all() -> Result<Vec<db_column::Model>, DaoError> {
    let db = DB_POOOOOOL.get().await;
    let result = db_column::Entity::find().all(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let models = result.unwrap();

    Ok(models)
}

pub async fn create(json_data: serde_json::Value) -> Result<db_column::Model, DaoError> {
    let db = DB_POOOOOOL.get().await;
    let model = db_column::ActiveModel::from_json(json_data);
    let inserted_model = model.unwrap().insert(db).await.unwrap();
    Ok(inserted_model)
}

pub async fn update(id: i32, json_data: serde_json::Value) -> Result<db_column::Model, DaoError> {
    let db = DB_POOOOOOL.get().await;
    let item: db_column::Model = db_column::Entity::find_by_id(id)
        .one(db)
        .await
        .unwrap()
        .unwrap();
    let mut item_active_model: db_column::ActiveModel = item.into();
    #[warn(unused_variables)]
    let result = item_active_model.set_from_json(json_data);
    Ok(item_active_model.update(db).await.unwrap())
}

pub async fn delete(id: i32) -> Result<bool, DaoError> {
    let db = DB_POOOOOOL.get().await;

    let item: Option<db_column::Model> = db_column::Entity::find_by_id(id).one(db).await.unwrap();

    match item {
        Some(value) => {
            let res: DeleteResult = value.delete(db).await.unwrap();
            assert_eq!(res.rows_affected, 1);
            Ok(res.rows_affected == 1)
        }
        None => Ok(false),
    }
}
