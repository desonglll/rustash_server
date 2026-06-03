use axum::Json;
use axum::extract::{Path, Query, State};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::error::AppError;
use crate::pagination::{PaginatedResponse, PaginationQuery};

#[derive(Deserialize)]
pub struct CreateFileTypeRequest {
    pub name: String,
    pub extensions: String,
}

#[derive(Deserialize)]
pub struct UpdateFileTypeRequest {
    pub name: Option<String>,
    pub extensions: Option<String>,
}

#[derive(Serialize)]
pub struct FileTypeResponse {
    pub id: i64,
    pub name: String,
    pub extensions: String,
}

impl From<crate::entity::file_type::Model> for FileTypeResponse {
    fn from(m: crate::entity::file_type::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            extensions: m.extensions,
        }
    }
}

pub async fn list(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<FileTypeResponse>>, AppError> {
    let (items, total) = state.file_type_service.list(&pagination).await?;
    let data = items.into_iter().map(FileTypeResponse::from).collect();
    Ok(Json(PaginatedResponse::new(data, total, &pagination)))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<FileTypeResponse>, AppError> {
    let item = state
        .file_type_service
        .get_by_id(id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(FileTypeResponse::from(item)))
}

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateFileTypeRequest>,
) -> Result<Json<FileTypeResponse>, AppError> {
    let item = state
        .file_type_service
        .create(req.name, req.extensions)
        .await?;
    Ok(Json(FileTypeResponse::from(item)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateFileTypeRequest>,
) -> Result<Json<FileTypeResponse>, AppError> {
    let item = state
        .file_type_service
        .update(id, req.name, req.extensions)
        .await?;
    Ok(Json(FileTypeResponse::from(item)))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<()>, AppError> {
    state.file_type_service.delete(id).await?;
    Ok(Json(()))
}
