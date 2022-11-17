use crate::{
    ControllerCommon,
    DaoUser::{self},
};
use warp::Reply;

pub async fn init_admin() -> () {
    let user_admin_result = DaoUser::get_by_name("admin".to_string()).await;
    if user_admin_result.is_err() {
        return;
    }

    let user_admin_opt = user_admin_result.unwrap();

    if user_admin_opt.is_none() {
        let data = r#"
    {
        "username": "admin",
        "password": "password",
        "email": "admin@admin.com",
        "first_name": "Admin",
        "last_name": "Admin"
    }"#;

        let value = serde_json::from_str(data).unwrap();
        let result = DaoUser::create(value).await;
        print!("{:?}", result);

        let role = r#"
        {"name": "admin"}
        "#;

        let permission_ = r#"
        {"name": "admin"}
        "#;
    }
}

pub async fn get_user(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoUser::get_by_id(id).await)
}

pub async fn get_user_by_name(name: String) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoUser::get_by_name(name).await)
}

pub async fn get_all_users() -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoUser::get_all().await)
}

pub async fn insert_user(json_data: serde_json::Value) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoUser::create(json_data).await)
}

pub async fn update_user(
    id: i32,
    json_data: serde_json::Value,
) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoUser::update(id, json_data).await)
}

pub async fn delete_user(id: i32) -> crate::GenericResult<impl Reply> {
    ControllerCommon::generate_response(DaoUser::delete(id).await)
}
