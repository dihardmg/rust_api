use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(message: &str, data: Option<T>) -> Self {
        Self {
            status: "success".to_string(),
            message: message.to_string(),
            data,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            status: "error".to_string(),
            message: message.to_string(),
            data: None,
        }
    }
}
