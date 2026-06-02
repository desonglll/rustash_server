use axum::{Json, extract::{Path, Query, State}, http::StatusCode, response::IntoResponse};
use uuid::Uuid;
use crate::AppState;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FileQuery {
    pub search: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>
}

pub async fn list_files(
    State(state): State<AppState>,
    Query(query): Query<FileQuery>,
) -> impl IntoResponse {
    match state.file_service.get_list(query).await {
        Ok(files) => (StatusCode::OK, Json(files)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": err.to_string() })),
        ).into_response(),
    }
}

pub async fn get_file(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.file_service.get_by_id(id).await {
        Ok(Some(file)) => (StatusCode::OK, Json(file)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "file not found" }))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": err.to_string() }))).into_response(),
    }
}

pub async fn delete_file(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.file_service.delete_by_id(id).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "result": "success" }))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": err.to_string() }))).into_response(),
    }
}
