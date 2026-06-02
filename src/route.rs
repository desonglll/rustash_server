use axum::{
    Router, routing::{delete, get, post, put},
};
use crate::{AppState, handler::scanner_handler::scan_file};
use crate::handler::file_handler::{list_files, get_file, delete_file};
use crate::handler::file_type_handler;
use crate::handler::stream_handler::stream_video;
use crate::handler::storage_root_handler;

pub fn create_routes(state: AppState) -> Router {
    let file_routes = Router::new()
        .route("/", get(list_files))
        .route("/{id}", get(get_file))
        .route("/{id}", delete(delete_file))
        .route("/{id}/stream", get(stream_video));

    let file_type_routes = Router::new()
        .route("/", get(file_type_handler::list))
        .route("/", post(file_type_handler::create))
        .route("/{id}", get(file_type_handler::get_by_id))
        .route("/{id}", put(file_type_handler::update))
        .route("/{id}", delete(file_type_handler::delete));

    let storage_root_routes = Router::new()
        .route("/", get(storage_root_handler::list))
        .route("/", post(storage_root_handler::create))
        .route("/{id}", get(storage_root_handler::get_by_id))
        .route("/{id}", put(storage_root_handler::update))
        .route("/{id}", delete(storage_root_handler::delete));

    Router::new()
        .route("/", get(|| async { "Welcome to Rust Stash Backend!" }))
        .route("/api/scan", post(scan_file))
        .nest("/api/files", file_routes)
        .nest("/api/file-types", file_type_routes)
        .nest("/api/storage-roots", storage_root_routes)
        .with_state(state)
}
