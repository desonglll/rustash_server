use std::net::SocketAddr;
use rustash_server::route;
use rustash_server::service::file_service;
use rustash_server::service::file_type_service;
use sea_orm::{Database};
use rustash_server::{config::AppConfig, scanner};
use migration::{Migrator, MigratorTrait};
use rustash_server::error::AppError;
use rustash_server::AppState;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let app_config = AppConfig::load().expect("failed to load configuration");
    println!("load configuration listening on port: {}", app_config.server.port);

    let database_url = format!("sqlite://{}?mode=rwc", app_config.database.file_path);
    let db = Database::connect(&database_url)
        .await
        .expect("can not connect to SQLite");
    println!("success to connect database: {}", app_config.database.file_path);
    println!("database_url: {}", database_url);
    let connection = sea_orm::Database::connect(&database_url).await?;
    Migrator::up(&connection, None).await.unwrap();


    let db_clone = db.clone();
    let scan_dirs = app_config.library.scan_directories.clone();
    tokio::spawn(async move {
        for dir in scan_dirs {
            if let Err(e) = scanner::scan_directory(&db_clone, &dir).await {
                eprintln!("scan [{}] failed: {}", dir, e);
            }
        }
    });

    let file_service = file_service::FileService::new(db.clone());
    let file_type_service = file_type_service::FileTypeService::new(db.clone());

    let state = AppState { db, config: app_config.clone(), file_service, file_type_service };
    let app = route::create_routes(state);


    let addr_str = format!("{}:{}", app_config.server.host, app_config.server.port);
    let addr: SocketAddr = addr_str.parse().expect("addr error");

    println!("server started at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
