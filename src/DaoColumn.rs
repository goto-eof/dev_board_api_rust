use crate::Structs::DaoError;
use crate::DB_POOL;
use chrono::Utc;
use entity::db_column;
use entity::db_item;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::FromQueryResult;
use sea_orm::IntoActiveModel;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;

pub async fn get_by_id(id: i32) -> Result<db_column::Model, DaoError> {
    let db = DB_POOL.get().await;
    let result = db_column::Entity::find_by_id(id).one(db).await;

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

pub async fn get_all() -> Result<Vec<db_column::Model>, DaoError> {
    let db = DB_POOL.get().await;
    let result = db_column::Entity::find()
        .order_by_asc(db_column::Column::Order)
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

#[derive(FromQueryResult, Debug)]
struct OptionResult {
    value: Option<i32>,
}

#[derive(FromQueryResult, Debug)]
struct MandatoryResult {
    value: i64,
}

pub async fn get_max_id() -> Result<i32, DaoError> {
    let db = DB_POOL.get().await;

    let result = db_column::Entity::find()
        .select_only()
        .column_as(db_column::Column::Id.max(), "value")
        .into_model::<OptionResult>()
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

    if count.value.is_none() {
        return Ok(0);
    }

    Ok(count.value.unwrap())
}

pub async fn get_count() -> Result<i64, DaoError> {
    let db = DB_POOL.get().await;

    let result = db_column::Entity::find()
        .select_only()
        .column_as(db_column::Column::Id.count(), "value")
        .into_model::<MandatoryResult>()
        .one(db)
        .await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error get_count(): {:?}", result.err()),
        });
    }
    let count_opt = result.unwrap();

    if count_opt.is_none() {
        return Ok(0);
    }

    Ok(count_opt.unwrap().value)
}

pub async fn create(json_data: serde_json::Value) -> Result<db_column::Model, DaoError> {
    let db = DB_POOL.get().await;
    let result = db_column::ActiveModel::from_json(json_data);

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let count_result = get_count().await;

    if count_result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error count2(): {:?}", count_result.err()),
        });
    }

    let count = count_result.unwrap();

    let mut model = result.unwrap();

    let dat = Utc::now().naive_utc();
    model.created_at = sea_orm::Set(Some(dat));
    model.updated_at = sea_orm::Set(Some(dat));
    model.order = sea_orm::Set(count);

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

pub async fn swap(idA: i32, idB: i32) -> Result<bool, DaoError> {
    let db = DB_POOL.get().await;
    let result_a = db_column::Entity::find_by_id(idA).one(db).await;
    let result_b = db_column::Entity::find_by_id(idB).one(db).await;

    if result_a.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result_a.err()),
        });
    }

    if result_b.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result_b.err()),
        });
    }

    let opt_a = result_a.unwrap();
    if opt_a.is_none() {
        return Err(DaoError {
            code: 2,
            err_type: crate::Structs::DaoErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let opt_b = result_b.unwrap();
    if opt_b.is_none() {
        return Err(DaoError {
            code: 2,
            err_type: crate::Structs::DaoErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let v_a = opt_a.unwrap();
    let v_b = opt_b.unwrap();

    let order_a = v_a.order;
    let order_b = v_b.order;

    let mut active_model_a = v_a.into_active_model();
    let mut active_model_b = v_b.into_active_model();
    active_model_a.order = sea_orm::Set(order_b);
    active_model_b.order = sea_orm::Set(order_a);
    let result_a = active_model_a.update(db).await;
    if result_a.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result_a.err()),
        });
    }
    let result_b = active_model_b.update(db).await;
    if result_b.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result_b.err()),
        });
    }
    Ok(true)
}

pub async fn update(id: i32, json_data: serde_json::Value) -> Result<db_column::Model, DaoError> {
    let db = DB_POOL.get().await;
    let result = db_column::Entity::find_by_id(id).one(db).await;

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

    let mut item_active_model: db_column::ActiveModel = opt.unwrap().into();

    let result = item_active_model.set_from_json(json_data);

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
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

    let result = db_column::Entity::find_by_id(id).one(db).await;

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

    let items_result = db_item::Entity::find()
        .filter(db_item::Column::ColumnId.eq(id))
        .all(db)
        .await;

    if items_result.is_err() {
        return Err(DaoError {
            code: 2,
            err_type: crate::Structs::DaoErrorType::Warning,
            message: format!("Error while retrieving items"),
        });
    }

    let items = items_result.unwrap();

    for item in items.into_iter() {
        let item_result = item.delete(db).await;
        if item_result.is_err() {
            return Err(DaoError {
                code: 2,
                err_type: crate::Structs::DaoErrorType::Warning,
                message: format!("Error while deleting item"),
            });
        }
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
