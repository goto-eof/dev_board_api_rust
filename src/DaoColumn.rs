use crate::DB_POOL;
use entity::db_column;
use sea_orm::ActiveModelTrait;
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
    let db = DB_POOL.get().await;
    let result = db_column::Entity::find_by_id(id).one(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

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
    let db = DB_POOL.get().await;
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
    let db = DB_POOL.get().await;
    let result = db_column::ActiveModel::from_json(json_data);

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let model = result.unwrap();

    let result = model.insert(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    Ok(result.unwrap())
}

pub async fn update(id: i32, json_data: serde_json::Value) -> Result<db_column::Model, DaoError> {
    let db = DB_POOL.get().await;
    let result = db_column::Entity::find_by_id(id).one(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DaoError {
            code: 2,
            err_type: DaoErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let mut item_active_model: db_column::ActiveModel = opt.unwrap().into();

    let result = item_active_model.set_from_json(json_data);

    if result.is_err() {
        if result.is_err() {
            return Err(DaoError {
                code: 1,
                err_type: DaoErrorType::Error,
                message: format!("DB Error: {:?}", result.err()),
            });
        }
    }

    let result = item_active_model.update(db).await;

    if result.is_err() {
        if result.is_err() {
            return Err(DaoError {
                code: 1,
                err_type: DaoErrorType::Error,
                message: format!("DB Error: {:?}", result.err()),
            });
        }
    }

    Ok(result.unwrap())
}

pub async fn delete(id: i32) -> Result<bool, DaoError> {
    let db = DB_POOL.get().await;

    let result = db_column::Entity::find_by_id(id).one(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DaoError {
            code: 2,
            err_type: DaoErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let result = opt.unwrap().delete(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let deletion_result = result.unwrap();
    assert_eq!(deletion_result.rows_affected, 1);
    Ok(deletion_result.rows_affected == 1)
}
