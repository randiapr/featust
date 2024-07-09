use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResponse<T> {
    pub response_code: u16,
    pub response_message: String,
    pub data: T,
}

impl<T> BaseResponse<T> {
    pub async fn new(
        response_code: Option<u16>,
        response_message: Option<String>,
        data: T,
    ) -> Self {
        let code = match response_code {
            Some(code) => code,
            None => StatusCode::OK.as_u16(),
        };
        let msg = match response_message {
            Some(msg) => msg,
            None => "Success".to_string(),
        };
        Self {
            response_code: code,
            response_message: msg,
            data,
        }
    }
}
