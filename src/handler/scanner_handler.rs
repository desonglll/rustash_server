

use axum::{Json, extract::{ State}, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{AppState,  scanner};

pub async fn scan_file(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let dirs = state.config.library.scan_directories.clone();
    let db_clone = state.db.clone();

    tokio::spawn(async move {
        println!("spawn scan task...");
        for d in dirs {
            if let Err(e) = scanner::scan_directory(&db_clone, &d).await {
                eprintln!("failed to scan [{}] : {}", d, e);
            }
        }
        println!("scan finished");
    });

    IntoResponse::into_response((
        StatusCode::ACCEPTED,
        Json(json!({"status": "scan task triggered in background"})),
    ))
}
