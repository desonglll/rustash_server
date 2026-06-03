use crate::handler::file_handler::{delete_file, get_file, list_files};
use crate::handler::file_type_handler;
use crate::handler::stream_handler::stream_video;
use crate::{AppState, handler::scanner_handler::scan_file};
use axum::{
    Router,
    http::{
        self, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    routing::{delete, get, post, put},
};
use tower_http::cors::CorsLayer;

pub fn create_routes(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("*".parse::<http::HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

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

    Router::new()
        .route("/", get(|| async { "Welcome to Rust Stash Backend!" }))
        .route("/api/scan", post(scan_file))
        .nest("/api/files", file_routes)
        .nest("/api/file-types", file_type_routes)
        .layer(cors)
        .with_state(state)
}
