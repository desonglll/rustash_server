pub mod config;
pub mod entity;
pub mod error;
pub mod handler;
pub mod pagination;
pub mod route;
pub mod scanner;
pub mod service;
use crate::{config::AppConfig, service::file_service, service::file_type_service};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: AppConfig,
    pub file_service: file_service::FileService,
    pub file_type_service: file_type_service::FileTypeService,
}
