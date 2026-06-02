use std::io::Read;
use std::path::Path;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use sea_orm::ActiveValue::Set;
use sha2::{Sha256, Digest};
use uuid::Uuid;
use walkdir::WalkDir;

use crate::entity;

struct DynamicExtensionRule {
    file_type_id: i64,
    extensions: Vec<String>,
}

/// Compute a fast fingerprint: SHA-256(file_size_le_bytes + first_1MB)
/// File size is included to make collisions practically impossible even
/// if two different files share the same first 1MB.
fn compute_fingerprint(file_path: &Path, file_size: u64) -> Option<String> {
    let mut file = std::fs::File::open(file_path).ok()?;
    let mut header = vec![0u8; 1024 * 1024]; // 1MB
    let bytes_read = file.read(&mut header).ok()?;
    header.truncate(bytes_read);

    let mut hasher = Sha256::new();
    hasher.update(file_size.to_le_bytes());
    hasher.update(&header);
    let result = hasher.finalize();
    Some(result.iter().map(|b| format!("{:02x}", b)).collect())
}

pub async fn scan_directory<P: AsRef<Path>>(
    db: &DatabaseConnection,
    root: &entity::storage_root::Model,
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

    let mount_path = Path::new(&root.mount_path);

    println!("start scan directory: {:?}", dir_path);

    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let current_ext = ext.to_lowercase();

                for rule in &scan_rules {
                    if rule.extensions.contains(&current_ext) {

                        let relative_path = path.strip_prefix(mount_path)
                            .unwrap_or(path)
                            .to_string_lossy()
                            .to_string();
                        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                        let file_size_bytes = entry.metadata()?.len();
                        let hash = compute_fingerprint(path, file_size_bytes);

                        // 1. Check by hash — file moved to new location
                        if let Some(ref h) = hash {
                            let by_hash = entity::file::Entity::find()
                                .filter(entity::file::Column::Hash.eq(h))
                                .one(db)
                                .await?;

                            if let Some(existing) = by_hash {
                                if existing.path != relative_path || existing.storage_root_id != root.id {
                                    println!("file moved, updating path [{}]: {} -> {}", existing.name, existing.path, relative_path);
                                    let mut active: entity::file::ActiveModel = existing.into();
                                    active.path = Set(relative_path);
                                    active.storage_root_id = Set(root.id);
                                    active.file_size = Set(file_size_bytes.to_string());
                                    active.hash = Set(Some(h.clone()));
                                    active.update(db).await?;
                                }
                                break;
                            }
                        }

                        // 2. Check by path + storage_root_id — file already known at this location
                        let by_path = entity::file::Entity::find()
                            .filter(entity::file::Column::Path.eq(&relative_path))
                            .filter(entity::file::Column::StorageRootId.eq(root.id))
                            .one(db)
                            .await?;

                        if let Some(existing) = by_path {
                            // Same path but no hash yet — fill it in
                            if existing.hash.is_none() {
                                let mut active: entity::file::ActiveModel = existing.into();
                                active.hash = Set(hash.clone());
                                active.update(db).await?;
                            }
                            break;
                        }

                        // 3. New file
                        println!("found new file [TypeID: {}]: {}", rule.file_type_id, file_name);

                        let new_file = entity::file::ActiveModel {
                            id: Set(Uuid::new_v4()),
                            name: Set(file_name.clone()),
                            path: Set(relative_path),
                            file_size: Set(file_size_bytes.to_string()),
                            file_type_id: Set(rule.file_type_id),
                            storage_root_id: Set(root.id),
                            hash: Set(hash),
                        };

                        new_file.insert(db).await?;
                        break;
                    }
                }
            }
        }
    }

    println!("scan finished: {:?}", dir_path);
    Ok(())
}
