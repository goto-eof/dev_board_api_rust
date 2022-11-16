use sea_orm::DbConn;
use serde_json::json;

use crate::DaoPermission;

pub async fn init_permissions(db: &DbConn) {
    let permissions = vec![
        "get_column",
        "get_all_columns",
        "get_all_columns_with_items",
        "insert_column",
        "update_column",
        "swap_columns",
        "delete_column",
        "get_item",
        "get_items",
        "insert_item",
        "update_item",
        "delete_item",
        "swap_items",
    ];

    for permission in permissions {
        let permission_model = DaoPermission::get_by_name(&permission).await;

        if permission_model.is_ok() && permission_model.unwrap().is_none() {
            let json_data = json!({ "name": permission });
            let json_data: serde_json::Value =
                serde_json::from_str(json_data.to_string().as_str()).unwrap();
            let permission_result = DaoPermission::create(json_data).await;
            println!("{:?}", permission_result)
        }
    }
}
