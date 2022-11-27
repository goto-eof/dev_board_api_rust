use crate::structure::structure::BoardFullResponse;
use crate::structure::structure::DashoardFullResponse;
use crate::structure::structure::DevBoardErrorType;
use crate::structure::structure::DevBoardGenericError;
use crate::util::util_authentication::extract_user_id;
use crate::DB_POOL;
use chrono::Utc;
use entity::db_board;
use entity::db_board_column;
use entity::db_board_user;
use entity::db_column;
use entity::db_item;
use log::debug;
use migration::DbErr;
use migration::JoinType;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::TransactionTrait;

pub async fn get_by_id(
    id: i32,
    jwt_opt: Option<String>,
) -> Result<Option<db_board::Model>, DevBoardGenericError> {
    let user_id = extract_user_id(jwt_opt.clone());
    if user_id.is_some() {
        let db = DB_POOL.get().await;
        let result = db_board::Entity::find_by_id(id)
            .join_rev(
                JoinType::InnerJoin,
                db_board_user::Entity::belongs_to(db_board::Entity)
                    .from(db_board_user::Column::BoardId)
                    .to(db_board::Column::Id)
                    .into(),
            )
            .filter(db_board_user::Column::UserId.eq(user_id.unwrap()))
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

        return Ok(Some(opt.unwrap()));
    }
    return Ok(None);
}

pub async fn share(
    board_id: i32,
    target_user_id: i32,
    _jwt_opt: Option<String>,
) -> Result<bool, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let dat = Utc::now().naive_utc();

    let shared_board_user = db_board_user::ActiveModel {
        board_id: sea_orm::Set(board_id),
        user_id: sea_orm::Set(target_user_id),
        created_at: sea_orm::Set(Some(dat)),
        updated_at: sea_orm::Set(Some(dat)),
        ..Default::default()
    };
    let result = shared_board_user.insert(db).await;
    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    Ok(true)
}

pub async fn unshare(
    board_id: i32,
    target_user_id: i32,
    _jwt_opt: Option<String>,
) -> Result<bool, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let board_user_result = db_board_user::Entity::find()
        .filter(db_board_user::Column::UserId.eq(target_user_id))
        .filter(db_board_user::Column::BoardId.eq(board_id))
        .all(db)
        .await;

    for board_user in board_user_result.unwrap() {
        let result = board_user.delete(db).await;
        if result.is_err() {
            return Err(DevBoardGenericError {
                success: false,
                code: 1,
                err_type: DevBoardErrorType::Error,
                message: format!("DB Error: {:?}", result.err()),
            });
        }
    }

    Ok(true)
}

pub async fn get_by_id_all(
    id: i32,
    jwt_opt: Option<String>,
) -> Result<Option<DashoardFullResponse>, DevBoardGenericError> {
    let user_id = extract_user_id(jwt_opt.clone());
    if user_id.is_some() {
        let db = DB_POOL.get().await;
        println!("{:?}", user_id);
        let dashboard_res_opt = db_board::Entity::find_by_id(id)
            .join_rev(
                JoinType::InnerJoin,
                db_board_user::Entity::belongs_to(db_board::Entity)
                    .from(db_board_user::Column::BoardId)
                    .to(db_board::Column::Id)
                    .into(),
            )
            .filter(db_board_user::Column::UserId.eq(user_id.unwrap()))
            .one(db)
            .await;

        println!("{:?}", dashboard_res_opt);

        if dashboard_res_opt.is_err() {
            return Err(DevBoardGenericError {
                success: false,
                code: 1,
                err_type: DevBoardErrorType::Error,
                message: format!("DB Error: {:?}", dashboard_res_opt.err()),
            });
        }

        let dashboard_res_opt = dashboard_res_opt.unwrap();

        if dashboard_res_opt.is_none() {
            return Err(DevBoardGenericError {
                success: false,
                code: 2,
                err_type: DevBoardErrorType::Warning,
                message: format!("Item not found (01)"),
            });
        }

        let dashboard_res_opt = dashboard_res_opt.unwrap();

        let result: Result<Vec<(db_column::Model, Vec<db_item::Model>)>, DbErr> =
            db_column::Entity::find()
                .join_rev(
                    JoinType::InnerJoin,
                    db_board_column::Entity::belongs_to(db_column::Entity)
                        .from(db_board_column::Column::ColumnId)
                        .to(db_column::Column::Id)
                        .into(),
                )
                .filter(db_board_column::Column::BoardId.eq(dashboard_res_opt.id))
                .order_by_asc(db_column::Column::Order)
                .find_with_related(db_item::Entity)
                .order_by_asc(db_item::Column::Order)
                .all(db)
                .await;

        println!("{:?}", result);
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

        let full_response: DashoardFullResponse = DashoardFullResponse {
            board: dashboard_res_opt,
            columns: boards_result,
        };

        return Ok(Some(full_response));
    }
    return Ok(None);
}

pub async fn get_all(
    jwt_opt: Option<String>,
) -> Result<Vec<db_board::Model>, DevBoardGenericError> {
    let user_id = extract_user_id(jwt_opt);
    if user_id.is_some() {
        let user_id = user_id.unwrap();
        let db = DB_POOL.get().await;

        let result = db_board::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                db_board_user::Entity::belongs_to(db_board::Entity)
                    .from(db_board_user::Column::BoardId)
                    .to(db_board::Column::Id)
                    .into(),
            )
            .filter(db_board_user::Column::UserId.eq(user_id))
            .order_by_asc(db_board_user::Column::Id)
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

        return Ok(models);
    }
    Ok(vec![])
}

pub async fn board_is_shared_with(
    board_id: i32,
    _jwt_opt: Option<String>,
) -> Result<Vec<i32>, DevBoardGenericError> {
    let db = DB_POOL.get().await;
    let result = db_board_user::Entity::find()
        .filter(db_board_user::Column::BoardId.eq(board_id))
        // .filter(db_board_user::Column::UserId.ne(user_id))
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
    let mut user_ids = vec![];

    for item in result.unwrap() {
        user_ids.push(item.user_id);
    }

    Ok(user_ids)
}

pub async fn create(
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> Result<db_board::Model, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let result = db
        .transaction::<_, db_board::Model, DbErr>(|txn| {
            Box::pin(async move {
                let user_id = extract_user_id(jwt_opt);
                let user_id = user_id.unwrap();

                let result = db_board::ActiveModel::from_json(json_data);

                let mut model = result.unwrap();

                let dat = Utc::now().naive_utc();
                model.created_at = sea_orm::Set(Some(dat));
                model.updated_at = sea_orm::Set(Some(dat));

                let result = model.insert(txn).await;

                let result = result.unwrap();

                let board_user = db_board_user::ActiveModel {
                    board_id: sea_orm::Set(result.id),
                    user_id: sea_orm::Set(user_id),
                    created_at: sea_orm::Set(Some(dat)),
                    updated_at: sea_orm::Set(Some(dat)),
                    ..Default::default()
                };

                let _board_user = board_user.insert(txn).await;

                return Ok(result);
            })
        })
        .await;
    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }
    Ok(result.unwrap())
}

pub async fn update(
    id: i32,
    json_data: serde_json::Value,
    _json_opt: Option<String>,
) -> Result<db_board::Model, DevBoardGenericError> {
    let db = DB_POOL.get().await;
    let result = db_board::Entity::find_by_id(id).one(db).await;

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

    let mut item_active_model: db_board::ActiveModel = opt.unwrap().into();

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

// TODO manage better unwrapping
// TODO in the future, use the DBMS to manage deletion (cascade: delete)
pub async fn delete(id: i32, jwt_opt: Option<String>) -> Result<bool, DevBoardGenericError> {
    let db_conn = DB_POOL.get().await;

    let result = db_conn
        .transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                let user_id = extract_user_id(jwt_opt).unwrap();

                // deleting board-user relationship
                let result = db_board_user::Entity::find()
                    .filter(db_board_user::Column::BoardId.eq(id))
                    .filter(db_board_user::Column::UserId.eq(user_id))
                    .one(txn)
                    .await;

                let res = result.unwrap().unwrap().delete(txn).await;
                if res.is_err() {
                    return Err(res.err().unwrap());
                }

                debug!("board-user deleted");

                // deleting board-column relationships
                let result = db_board_column::Entity::find()
                    .filter(db_board_column::Column::BoardId.eq(id))
                    .all(txn)
                    .await;
                let mut columns_id: Vec<i32> = vec![];
                for board_column in result.unwrap() {
                    columns_id.push(board_column.column_id);
                    let res = board_column.delete(txn).await;
                    if res.is_err() {
                        return Err(res.err().unwrap());
                    }
                }

                debug!("board-column deleted");

                // deleting board
                debug!("board id: {}", id);

                let result = db_board::Entity::find_by_id(id).one(txn).await;
                debug!("board: {:?}", result);

                let opt = result.unwrap();

                let res = opt.unwrap().delete(txn).await;
                if res.is_err() {
                    return Err(res.err().unwrap());
                }

                debug!("board deleted");

                // deleting items relationships
                let items = db_item::Entity::find()
                    .filter(db_item::Column::ColumnId.is_in(columns_id.clone()))
                    .all(txn)
                    .await;

                for column_item in items.unwrap() {
                    let res = column_item.delete(txn).await;
                    if res.is_err() {
                        return Err(res.err().unwrap());
                    }
                }

                debug!("items deleted");

                // deleting columns
                let columns = db_column::Entity::find()
                    .filter(db_column::Column::Id.is_in(columns_id))
                    .all(txn)
                    .await;

                for column in columns.unwrap() {
                    let res = column.delete(txn).await;
                    if res.is_err() {
                        return Err(res.err().unwrap());
                    }
                }
                debug!("columns deleted");

                Ok(())
            })
        })
        .await;
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
    Ok(true)
}
