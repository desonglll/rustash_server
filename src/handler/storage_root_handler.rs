use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;
use crate::pagination::{PaginatedResponse, PaginationQuery};
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateStorageRootRequest {
    pub name: String,
    pub mount_path: String,
    pub volume_uuid: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateStorageRootRequest {
    pub name: Option<String>,
    pub mount_path: Option<String>,
    pub volume_uuid: Option<String>,
}

#[derive(Serialize)]
pub struct StorageRootResponse {
    pub id: Uuid,
    pub name: String,
    pub mount_path: String,
    pub volume_uuid: Option<String>,
}

impl From<crate::entity::storage_root::Model> for StorageRootResponse {
    fn from(m: crate::entity::storage_root::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            mount_path: m.mount_path,
            volume_uuid: m.volume_uuid,
        }
    }
}

pub async fn list(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<StorageRootResponse>>, AppError> {
    let (items, total) = state.storage_root_service.list(&pagination).await?;
    let data = items.into_iter().map(StorageRootResponse::from).collect();
    Ok(Json(PaginatedResponse::new(data, total, &pagination)))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<StorageRootResponse>, AppError> {
    let item = state
        .storage_root_service
        .get_by_id(id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(StorageRootResponse::from(item)))
}

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateStorageRootRequest>,
) -> Result<Json<StorageRootResponse>, AppError> {
    let item = state
        .storage_root_service
        .create(req.name, req.mount_path, req.volume_uuid)
        .await?;
    Ok(Json(StorageRootResponse::from(item)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateStorageRootRequest>,
) -> Result<Json<StorageRootResponse>, AppError> {
    let item = state
        .storage_root_service
        .update(id, req.name, req.mount_path, req.volume_uuid)
        .await?;
    Ok(Json(StorageRootResponse::from(item)))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    state.storage_root_service.delete(id).await?;
    Ok(Json(()))
}
