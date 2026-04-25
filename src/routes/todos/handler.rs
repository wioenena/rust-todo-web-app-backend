use axum::extract::OriginalUri;

use crate::wrappers::api_response::ApiResponse;

#[utoipa::path(
        get,
        path = "/",
        responses(
            (status = 201, description = "Yeni bir görev oluşturur", body = ApiResponse<i32>)
        )
    )]
pub async fn get_todos(uri: OriginalUri) -> Result<ApiResponse<i32>, String> {
    Err(uri.path().into())
}
