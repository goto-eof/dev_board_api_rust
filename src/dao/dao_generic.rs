use migration::{DbErr, Iden};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, TryIntoModel};
use serde::{Deserialize, Serialize};

use crate::DB_POOL;

pub(crate) async fn get_by_id<ModelType: EntityTrait>(
    id: <<ModelType as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType,
) -> Result<Option<ModelType::Model>, DbErr> {
    let db = DB_POOL.get().await;
    return ModelType::find_by_id(id).one(db).await;
}

pub(crate) async fn get_all<ModelType: EntityTrait>() -> Result<Vec<ModelType::Model>, DbErr> {
    let db = DB_POOL.get().await;
    return ModelType::find().all(db).await;
}

pub(crate) async fn insert_value<
    ActiveModelType: std::marker::Send + sea_orm::ActiveModelBehavior,
>(
    json_string: String,
) -> Result<String, DbErr>
where
    <<ActiveModelType as ActiveModelTrait>::Entity as EntityTrait>::Model:
        IntoActiveModel<ActiveModelType>,
    for<'de> <<ActiveModelType as ActiveModelTrait>::Entity as EntityTrait>::Model:
        Deserialize<'de> + Serialize,
{
    let db = DB_POOL.get().await;
    let json_value = serde_json::from_str(&json_string).unwrap();
    let to_insert = ActiveModelType::from_json(json_value)?;
    let result = to_insert.insert(db).await;
    let model = result.unwrap().try_into_model()?;

    return Ok(serde_json::to_string(&model).unwrap());
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    name: String,
}

// pub async fn update<
//     ActiveModelType: ActiveModelTrait,
//     ModelType: EntityTrait + sea_orm::ActiveModelTrait,
//     Third: IntoActiveModel<ModelType>,
// >(
//     id: i32,
//     json_string: String,
// ) -> Result<String, DbErr>
// where
//     <<ActiveModelType as ActiveModelTrait>::Entity as EntityTrait>::Model:
//         IntoActiveModel<ActiveModelType>,
//     for<'de> <<ActiveModelType as ActiveModelTrait>::Entity as EntityTrait>::Model:
//         Deserialize<'de> + Serialize,
// {
//     println!("____JSON: {:?}", json_string);
//     let db = DB_POOL.get().await;
//     let json_value = serde_json::from_str(&json_string).unwrap();

//     let model = ModelType::find_by_id(id).one(db).await;

//     let result = model
//         .unwrap()
//         .unwrap()
//         .into_active_model()
//         .update(db, json_value)
//         .await
//         .unwrap();
//     result = result.unwrap();

//     return Ok(serde_json::to_string(&result).unwrap());
// }

// pub(crate) async fn update_new<
//     ActiveModelType: std::marker::Send
//         + sea_orm::ActiveModelBehavior
//         + ActiveModelTrait
//         + IntoActiveModel<ActiveModelType>
//         + std::convert::From<<ModelType as sea_orm::EntityTrait>::Model>,
//     ModelType: EntityTrait + std::convert::From<<ModelType as sea_orm::EntityTrait>::Model>,
// >(
//     id: <<ModelType as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType,
//     json_string: String,
// ) -> Result<String, DbErr>
// where
//     // <<ActiveModelType as ActiveModelTrait>::Entity as EntityTrait>::Model:
//     //     IntoActiveModel<ActiveModelType>,
//     // for<'de> <<ActiveModelType as ActiveModelTrait>::Entity as EntityTrait>::Model:
//     //     Deserialize<'de> + Serialize,
//     // <ModelType as EntityTrait>::Model: IntoActiveModel<<ModelType as EntityTrait>::Model>,
//     // <ModelType as EntityTrait>::Model: ActiveModelTrait,
//     for<'de> <<<ModelType as EntityTrait>::Model as ActiveModelTrait>::Entity as EntityTrait>::Model:
//         Deserialize<'de>,
//     <<<ModelType as EntityTrait>::Model as ActiveModelTrait>::Entity as EntityTrait>::Model:
//         IntoActiveModel<<ModelType as EntityTrait>::Model>,
//     <<<ModelType as EntityTrait>::Model as ActiveModelTrait>::Entity as EntityTrait>::Model:
//         Serialize,
//     <ModelType as EntityTrait>::Model: ActiveModelBehavior,
// {
//     println!("____JSON: {:?}", json_string);
//     let db = DB_POOL.get().await;
//     let result = ModelType::find_by_id(id).one(db).await;
//     let model = result.unwrap().unwrap();
//     let mut active_model = model.into_active_model();
//     let res = active_model.set_from_json(sea_orm::JsonValue::String(json_string));
//     println!("____RES: {:?}", res);
//     let result = active_model.update(db).await;

//     println!("_______________{:?}", result);
//     let model = result.unwrap().try_into_model()?;
//     return Ok(serde_json::to_string(&model).unwrap());
// }

pub(crate) async fn delete<ModelType: EntityTrait>(
    id: <<ModelType as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType,
) -> Result<bool, DbErr> {
    let db = DB_POOL.get().await;
    let res = ModelType::delete_by_id(id).exec(db).await?;
    Ok(res.rows_affected == 1)
}
