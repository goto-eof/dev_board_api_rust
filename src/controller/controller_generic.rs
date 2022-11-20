use crate::dao::dao_generic;

use super::controller_common;
use entity::db_board;
use serde::{Deserialize, Serialize};
use warp::Reply;

pub async fn get_generic(id: i32) -> crate::GenericResult<impl Reply> {
    let result = dao_generic::get_by_id::<db_board::Entity>(id).await;
    controller_common::generate_response(Ok(result.unwrap()))
}

pub async fn get_all_generics() -> crate::GenericResult<impl Reply> {
    let result = dao_generic::get_all::<db_board::Entity>().await;
    controller_common::generate_response(Ok(result.unwrap()))
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    name: String,
}

pub async fn insert_generic(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    let json_data = Board {
        name: "Ciao".to_string(),
    };
    let json_data = serde_json::to_value(json_data).unwrap().to_string();
    let _result = dao_generic::insert_value::<db_board::ActiveModel>(json_data).await;
    controller_common::generate_response(Ok("ciao"))
}

// TODO: Implement update
// pub async fn update_generic(
//     id: i32,
//     json_data: serde_json::Value,
// ) -> crate::GenericResult<impl Reply> {
//     let json_data = Board {
//         name: "CiaoCioa".to_string(),
//     };
//     let json_data = serde_json::to_value(json_data).unwrap().to_string();
//     let _result =
//         dao_generic::update::<db_board::ActiveModel, db_board::Entity, db_board::ActiveModel>(
//             id, json_data,
//         )
//         .await;
//     controller_common::generate_response(Ok("ciao"))
// }

pub async fn delete_generic(id: i32) -> crate::GenericResult<impl Reply> {
    let result = dao_generic::delete::<db_board::Entity>(id).await;
    controller_common::generate_response(Ok(result.unwrap()))
}
