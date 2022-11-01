use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum DaoErrorType {
    Error,
    Warning,
}
#[derive(Serialize, Debug)]
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
