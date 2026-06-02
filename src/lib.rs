
pub mod config;
pub mod scanner;
pub mod entity;
pub mod handler;
pub mod service;
pub mod error;
pub mod route;
pub mod pagination;
use sea_orm::{DatabaseConnection};
use crate::{config::AppConfig, service::file_service, service::file_type_service};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: AppConfig,
    pub file_service: file_service::FileService,
    pub file_type_service: file_type_service::FileTypeService,
}
