use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::file;
use crate::error::AppError;
use crate::pagination::{PaginatedResponse, PaginationQuery};
use crate::AppState;

#[derive(Deserialize)]
pub struct FileQuery {
    pub search: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationQuery,
}

#[derive(Serialize)]
pub struct FileResponse {
    pub id: String,
    pub name: String,
    pub path: String,
    pub file_size: String,
    pub file_type_id: i64,
}

impl From<file::Model> for FileResponse {
    fn from(m: file::Model) -> Self {
        Self {
            id: m.id.to_string(),
            name: m.name,
            path: m.path,
            file_size: m.file_size,
            file_type_id: m.file_type_id,
        }
    }
}

pub async fn list_files(
    State(state): State<AppState>,
    Query(query): Query<FileQuery>,
) -> Result<Json<PaginatedResponse<FileResponse>>, AppError> {
    let (items, total) = state
        .file_service
        .get_list(query.search, &query.pagination)
        .await?;
    let data = items.into_iter().map(FileResponse::from).collect();
    Ok(Json(PaginatedResponse::new(data, total, &query.pagination)))
}

pub async fn get_file(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<FileResponse>, AppError> {
    let item = state
        .file_service
        .get_by_id(id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(FileResponse::from(item)))
}

pub async fn delete_file(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    state.file_service.delete_by_id(id).await?;
    Ok(Json(()))
}
