use crate::Structs::DaoError;
use crate::DB_POOL;
use chrono::Utc;
use entity::db_item;
use log::debug;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::FromQueryResult;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;

pub async fn get_by_id(id: i32) -> Result<db_item::Model, DaoError> {
    let db = DB_POOL.get().await;
    let result = db_item::Entity::find_by_id(id).one(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DaoError {
            code: 2,
            err_type: crate::Structs::DaoErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    Ok(opt.unwrap())
}

pub async fn get_all() -> Result<Vec<db_item::Model>, DaoError> {
    let db = DB_POOL.get().await;
    let result = db_item::Entity::find().all(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let models = result.unwrap();

    Ok(models)
}

pub async fn get_by_parent_id(parent_id: i32) -> Result<Vec<db_item::Model>, DaoError> {
    let db = DB_POOL.get().await;

    let result = db_item::Entity::find()
        .filter(db_item::Column::ColumnId.eq(parent_id))
        .order_by_asc(db_item::Column::Id)
        .all(db)
        .await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let models = result.unwrap();

    Ok(models)
}

pub async fn create(json_data: serde_json::Value) -> Result<db_item::Model, DaoError> {
    let db = DB_POOL.get().await;
    let result = db_item::ActiveModel::from_json(json_data);

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let mut model = result.unwrap();

    let dat = Utc::now().naive_utc();

    model.created_at = sea_orm::Set(Some(dat));
    model.updated_at = sea_orm::Set(Some(dat));

    let max_id_result = get_max_id().await;

    match max_id_result {
        Ok(value) => model.code = sea_orm::Set(format!("{}", value)),
        Err(err) => {
            return Err(err);
        }
    }

    let result = model.insert(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    Ok(result.unwrap())
}

#[derive(FromQueryResult, Debug)]
struct CountResult {
    max: Option<i32>,
}

pub async fn get_max_id() -> Result<i32, DaoError> {
    let db = DB_POOL.get().await;

    let result = db_item::Entity::find()
        .select_only()
        .column_as(db_item::Column::Id.max(), "max")
        .into_model::<CountResult>()
        .one(db)
        .await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }
    let count = result.unwrap().unwrap();

    if count.max.is_none() {
        return Ok(0);
    }

    debug!("{:?}", count);

    Ok(count.max.unwrap())
}

pub async fn update(id: i32, json_data: serde_json::Value) -> Result<db_item::Model, DaoError> {
    let db = DB_POOL.get().await;
    let result = db_item::Entity::find_by_id(id).one(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DaoError {
            code: 2,
            err_type: crate::Structs::DaoErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let mut item_active_model: db_item::ActiveModel = opt.unwrap().into();

    let result = item_active_model.set_from_json(json_data);

    if result.is_err() {
        if result.is_err() {
            return Err(DaoError {
                code: 1,
                err_type: crate::Structs::DaoErrorType::Error,
                message: format!("DB Error: {:?}", result.err()),
            });
        }
    }

    let dat = Utc::now().naive_utc();
    item_active_model.updated_at = sea_orm::Set(Some(dat));

    let result = item_active_model.update(db).await;

    if result.is_err() {
        if result.is_err() {
            return Err(DaoError {
                code: 1,
                err_type: crate::Structs::DaoErrorType::Error,
                message: format!("DB Error: {:?}", result.err()),
            });
        }
    }

    Ok(result.unwrap())
}

pub async fn delete(id: i32) -> Result<bool, DaoError> {
    let db = DB_POOL.get().await;

    let result = db_item::Entity::find_by_id(id).one(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DaoError {
            code: 2,
            err_type: crate::Structs::DaoErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let result = opt.unwrap().delete(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let deletion_result = result.unwrap();
    assert_eq!(deletion_result.rows_affected, 1);
    Ok(deletion_result.rows_affected == 1)
}
