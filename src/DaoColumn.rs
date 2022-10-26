use crate::DB_POOOOOOL;
use entity::db_column;
use sea_orm::ActiveModelTrait;
use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
type GenericResult<T> = std::result::Result<T, ()>;

pub async fn get_by_id(id: i32) -> GenericResult<db_column::Model> {
    let db = DB_POOOOOOL.get().await;

    let item: db_column::Model = db_column::Entity::find_by_id(id)
        .one(db)
        .await
        .unwrap()
        .unwrap();

    Ok(item)
}

pub async fn get_all() -> GenericResult<Vec<db_column::Model>> {
    let db = DB_POOOOOOL.get().await;
    let models: Vec<db_column::Model> = db_column::Entity::find().all(db).await.unwrap();
    Ok(models)
}

pub async fn create(json_data: serde_json::Value) -> GenericResult<db_column::Model> {
    let db = DB_POOOOOOL.get().await;
    let model = db_column::ActiveModel::from_json(json_data);
    let inserted_model = model.unwrap().insert(db).await.unwrap();
    Ok(inserted_model)
}

pub async fn update(id: i32, json_data: serde_json::Value) -> GenericResult<db_column::Model> {
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

pub async fn delete(id: i32) -> GenericResult<bool> {
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
