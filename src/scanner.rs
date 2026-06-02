use std::path::Path;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use walkdir::WalkDir;

use crate::entity;

struct DynamicExtensionRule {
    file_type_id: i64,
    extensions: Vec<String>,
}

pub async fn scan_directory<P: AsRef<Path>>(
    db: &DatabaseConnection,
    dir_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let raw_dir_path = dir_path.as_ref().to_string_lossy();
    let expanded_raw_path = shellexpand::full(&raw_dir_path)?;
    let dir_path = Path::new(expanded_raw_path.as_ref());

    if !dir_path.exists() {
        return Err(format!("folder not exists: {:?}", dir_path).into());
    }

    let all_types = entity::file_type::Entity::find().all(db).await?;

    let all_types = if all_types.is_empty() {
        println!("empty file type, init with video...");
        let default_type = entity::file_type::ActiveModel {
            id: Set(0),
            name: Set("video".to_string()),
            extensions: Set("mp4,mkv,avi,mov".to_string()),
        };
        vec![default_type.insert(db).await?]
    } else {
        all_types
    };

    let mut scan_rules: Vec<DynamicExtensionRule> = Vec::new();
    for t in all_types {
        let exts: Vec<String> = t.extensions
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();

        scan_rules.push(DynamicExtensionRule {
            file_type_id: t.id,
            extensions: exts,
        });
    }

    println!("start scan directory: {:?}", dir_path);

    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let current_ext = ext.to_lowercase();

                for rule in &scan_rules {
                    if rule.extensions.contains(&current_ext) {

                        let absolute_path = path.to_string_lossy().to_string();
                        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                        let file_size_bytes = entry.metadata()?.len() as i64;

                        let exists = entity::file::Entity::find()
                            .filter(entity::file::Column::Path.eq(&absolute_path))
                            .one(db)
                            .await?;

                        if exists.is_none() {
                            println!("found new file [TypeID: {}]: {}", rule.file_type_id, file_name);

                            let new_file = entity::file::ActiveModel {
                                id: Set(Uuid::new_v4()),
                                name: Set(file_name.clone()),
                                path: Set(absolute_path),
                                file_size: Set(file_size_bytes.to_string()),
                                file_type_id: Set(rule.file_type_id),
                            };

                            new_file.insert(db).await?;
                        }
                        break;
                    }
                }
            }
        }
    }

    println!("scan finished: {:?}", dir_path);
    Ok(())
}
