use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub message: String,
    pub status: u16,
    #[serde(serialize_with = "serialize_ts_as_string")]
    pub timestamp: DateTime<Utc>,
    pub data: Option<T>,
    pub errors: Vec<String>,
    pub trace_id: Option<uuid::Uuid>,
    pub path: String,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn new(
        success: bool,
        message: impl Into<String>,
        status: u16,
        data: Option<T>,
        errors: Option<Vec<String>>,
        trace_id: Option<uuid::Uuid>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            success,
            message: message.into(),
            status,
            timestamp: Utc::now(),
            data,
            errors: errors.unwrap_or_default(),
            trace_id,
            path: path.into(),
        }
    }

    pub fn ok(message: impl Into<String>, status: u16, data: T, path: impl Into<String>) -> Self {
        Self::new(true, message, status, Some(data), None, None, path)
    }

    #[allow(unused)]
    pub fn err(
        message: impl Into<String>,
        status: u16,
        errors: Option<Vec<String>>,
        trace_id: uuid::Uuid,
        path: impl Into<String>,
    ) -> Self {
        Self::new(false, message, status, None, errors, Some(trace_id), path)
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (status, Json(self)).into_response()
    }
}

pub fn serialize_ts_as_string<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.timestamp_millis().to_string())
}
