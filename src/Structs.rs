use serde::Serialize;

#[derive(Serialize)]
pub enum DaoErrorType {
    Error,
    Warning,
}
#[derive(Serialize)]
pub struct DaoError {
    pub code: i32,
    pub err_type: DaoErrorType,
    pub message: String,
}

#[derive(Serialize)]
pub struct Response<T> {
    pub success: bool,
    pub result: T,
}
