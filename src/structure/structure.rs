use entity::{db_board, db_column, db_item};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Debug)]
pub enum DevBoardErrorType {
    Error,
    Warning,
}
#[derive(Serialize, Debug)]
pub struct DevBoardGenericError {
    pub(crate) success: bool,
    pub code: i32,
    pub err_type: DevBoardErrorType,
    pub message: String,
}

#[derive(Serialize)]
pub struct Response<T> {
    pub success: bool,
    pub result: T,
    pub refresh_token: bool,
}

#[derive(Clone, Deserialize)]
pub struct SwapRequest {
    pub id_a: i32,
    pub id_b: i32,
}

#[derive(Serialize)]
pub struct BoardsFullResponse {
    pub columns: Vec<BoardFullResponse>,
}

#[derive(Serialize)]
pub struct BoardFullResponse {
    pub column: db_column::Model,
    pub items: Vec<db_item::Model>,
}

#[derive(Serialize)]
pub struct DashoardFullResponse {
    pub(crate) board: db_board::Model,
    pub columns: Vec<BoardFullResponse>,
}

#[derive(Serialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}
#[derive(Serialize)]
pub struct LogoutResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Jwt {
    pub jwt: Option<String>,
}

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

#[derive(Serialize)]
pub struct SharedWithResponse {
    pub board: db_board::Model,
    pub users: Vec<UserReponse>,
}

#[derive(Serialize)]
pub struct UserReponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
