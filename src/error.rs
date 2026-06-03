use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("failed to load configuration: {0}")]
    Config(#[from] config::ConfigError),

    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("not found")]
    NotFound,

    #[error("network address error: {0}")]
    AddrParse(#[from] std::net::AddrParseError),
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            AppError::NotFound => axum::http::StatusCode::NOT_FOUND,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}
