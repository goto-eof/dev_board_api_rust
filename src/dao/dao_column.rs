use crate::structure::structure::BoardFullResponse;
use crate::structure::structure::BoardsFullResponse;
use crate::structure::structure::DevBoardErrorType;
use crate::structure::structure::DevBoardGenericError;
use crate::structure::structure::SwapRequest;
use crate::util::util_authentication::extract_user_id;
use crate::DB_POOL;
use chrono::Utc;
use entity::db_board_column;
use entity::db_column;
use entity::db_item;
use migration::DbErr;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::FromQueryResult;
use sea_orm::IntoActiveModel;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;

pub async fn get_by_id(id: i32) -> Result<db_column::Model, DevBoardGenericError> {
    let db = DB_POOL.get().await;
    let result = db_column::Entity::find_by_id(id).one(db).await;

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

pub async fn get_all() -> Result<Vec<db_column::Model>, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let result = db_column::Entity::find()
        .order_by_asc(db_column::Column::Order)
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

pub async fn get_all_with_items() -> Result<BoardsFullResponse, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let result: Result<Vec<(db_column::Model, Vec<db_item::Model>)>, DbErr> =
        db_column::Entity::find()
            .order_by_asc(db_column::Column::Order)
            .find_with_related(db_item::Entity)
            .order_by_asc(db_item::Column::Order)
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

    let mut boards_result: Vec<BoardFullResponse> = Vec::new();
    for board_tuple in models {
        let board_struct: BoardFullResponse = BoardFullResponse {
            column: board_tuple.0,
            items: board_tuple.1,
        };
        boards_result.push(board_struct);
    }

    let full_response: BoardsFullResponse = BoardsFullResponse {
        columns: boards_result,
    };

    Ok(full_response)
}

#[derive(FromQueryResult, Debug)]
struct OptionResult {
    value: Option<i64>,
}

pub async fn get_next_order_number() -> Result<i64, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let result = db_column::Entity::find()
        .select_only()
        .column_as(db_column::Column::Order.max(), "value")
        .into_model::<OptionResult>()
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
    let count = result.unwrap().unwrap();

    if count.value.is_none() {
        return Ok(0);
    }

    Ok(count.value.unwrap() + 1)
}

// TODO make transactional
// TODO make sure that the column is associated to the loggedin user
pub async fn create(
    board_id: i32,
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> Result<db_column::Model, DevBoardGenericError> {
    let db = DB_POOL.get().await;
    let result = db_column::ActiveModel::from_json(json_data);
    let user_id = extract_user_id(jwt_opt).unwrap();
    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let next_order_number = get_next_order_number().await;

    if next_order_number.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error count2(): {:?}", next_order_number.err()),
        });
    }

    let count = next_order_number.unwrap();

    let mut model = result.unwrap();

    let dat = Utc::now().naive_utc();
    model.created_at = sea_orm::Set(Some(dat));
    model.updated_at = sea_orm::Set(Some(dat));
    model.order = sea_orm::Set(count);

    let result = model.insert(db).await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let result = result.unwrap();

    let board_user_am = db_board_column::ActiveModel {
        board_id: sea_orm::Set(board_id),
        column_id: sea_orm::Set(result.id),
        created_at: sea_orm::Set(Some(dat)),
        updated_at: sea_orm::Set(Some(dat)),
        ..Default::default()
    };

    board_user_am.insert(db).await;

    Ok(result)
}

pub async fn swap(swap_request: SwapRequest) -> Result<bool, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let result_a = db_column::Entity::find_by_id(swap_request.id_a)
        .one(db)
        .await;
    let result_b = db_column::Entity::find_by_id(swap_request.id_b)
        .one(db)
        .await;

    if result_a.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result_a.err()),
        });
    }

    if result_b.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result_b.err()),
        });
    }

    let opt_a = result_a.unwrap();
    if opt_a.is_none() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let opt_b = result_b.unwrap();
    if opt_b.is_none() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
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
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result_a.err()),
        });
    }
    let result_b = active_model_b.update(db).await;
    if result_b.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result_b.err()),
        });
    }
    Ok(true)
}

pub async fn update(
    id: i32,
    json_data: serde_json::Value,
) -> Result<db_column::Model, DevBoardGenericError> {
    let db = DB_POOL.get().await;
    let result = db_column::Entity::find_by_id(id).one(db).await;

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

    let mut item_active_model: db_column::ActiveModel = opt.unwrap().into();

    let result = item_active_model.set_from_json(json_data);

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
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

pub async fn delete(id: i32) -> Result<bool, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let result = db_column::Entity::find_by_id(id).one(db).await;

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

    let items_result = db_item::Entity::find()
        .filter(db_item::Column::ColumnId.eq(id))
        .all(db)
        .await;

    if items_result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
            message: format!("Error while retrieving items"),
        });
    }

    let items = items_result.unwrap();

    for item in items.into_iter() {
        let item_result = item.delete(db).await;
        if item_result.is_err() {
            return Err(DevBoardGenericError {
                success: false,
                code: 2,
                err_type: DevBoardErrorType::Warning,
                message: format!("Error while deleting item"),
            });
        }
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
