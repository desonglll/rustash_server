use std::path::Path;
use std::fs;
use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub library: LibraryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryConfig {
    pub scan_directories: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                port: 8080,
                host: "127.0.0.1".to_string(),
            },
            database: DatabaseConfig {
                file_path: "stash.sqlite".to_string(),
            },
            library: LibraryConfig {
                scan_directories: vec![
                    if cfg!(target_os = "windows") { "C:\\Videos".to_string() } else { "$HOME/Movies".to_string() }
                ],
            },
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Path::new("config.toml");

        if !config_path.exists() {
            let default_config = AppConfig::default();
            let toml_string = toml::to_string_pretty(&default_config)?;
            fs::write(config_path, toml_string)?;
            println!("generated default: config.toml");
        }

        let settings = config::Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .build()?;

        let app_config: AppConfig = settings.try_deserialize()?;
        Ok(app_config)
    }
}
