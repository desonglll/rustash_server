

use axum::{Json, extract::{ State}, http::StatusCode, response::IntoResponse};
use sea_orm::EntityTrait;
use serde_json::json;

use crate::{AppState, entity, scanner};

pub async fn scan_file(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let storage_roots = entity::storage_root::Entity::find().all(&state.db).await.unwrap();

    for root in storage_roots {
        println!("processing storage root [{}: {}]...", root.name, root.mount_path);
        let db_clone = state.db.clone();
        let dirs = state.config.library.scan_directories.clone();
        tokio::spawn(async move {
            println!("spawn scan task...");
            for d in dirs {
                if let Err(e) = scanner::scan_directory(&db_clone, &root, &root.mount_path).await {
                    eprintln!("failed to scan [{}, {}] : {}", &root.mount_path, &root.id, e);
                }
            }
            println!("scan finished");
        });
    }



    IntoResponse::into_response((
        StatusCode::ACCEPTED,
        Json(json!({"status": "scan task triggered in background"})),
    ))
}
