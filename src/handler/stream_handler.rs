use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::{StatusCode, header},
    response::IntoResponse,
};
use tower_http::services::ServeFile;
use uuid::Uuid;

pub async fn stream_video(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.file_service.get_absolute_path(id).await {
        Ok(Some(physical_path)) => {
            let path = std::path::Path::new(&physical_path);

            if !path.exists() {
                return IntoResponse::into_response((
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({ "error": "video not exists" })),
                ));
            }

            match ServeFile::new(path)
                .try_call(axum::http::Request::new(axum::body::Body::empty()))
                .await
            {
                Ok(response) => {
                    let mut res = response.into_response();
                    res.headers_mut().insert(
                        header::CONTENT_DISPOSITION,
                        header::HeaderValue::from_static("inline"),
                    );
                    res
                }
                Err(_) => IntoResponse::into_response((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": "failed to read video stream" })),
                )),
            }
        }
        Ok(None) => IntoResponse::into_response((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "not found" })),
        )),
        Err(e) => IntoResponse::into_response((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )),
    }
}
