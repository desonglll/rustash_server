use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect, Set};

use crate::entity::storage_root;
use crate::pagination::PaginationQuery;

#[derive(Clone)]
pub struct StorageRootService {
    db: DatabaseConnection,
}

impl StorageRootService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(
        &self,
        pagination: &PaginationQuery,
    ) -> Result<(Vec<storage_root::Model>, u64), sea_orm::DbErr> {
        let total = storage_root::Entity::find()
            .count(&self.db)
            .await?;

        let items = storage_root::Entity::find()
            .limit(pagination.per_page())
            .offset(pagination.offset())
            .all(&self.db)
            .await?;

        Ok((items, total))
    }

    pub async fn get_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<storage_root::Model>, sea_orm::DbErr> {
        storage_root::Entity::find_by_id(id)
            .one(&self.db)
            .await
    }

    pub async fn create(
        &self,
        name: String,
        mount_path: String,
        volume_uuid: Option<String>,
    ) -> Result<storage_root::Model, sea_orm::DbErr> {
        let id = uuid::Uuid::new_v4();
        let model = storage_root::ActiveModel {
            id: Set(id),
            name: Set(name),
            mount_path: Set(mount_path),
            volume_uuid: Set(volume_uuid),
        };
        model.insert(&self.db).await
    }

    pub async fn update(
        &self,
        id: uuid::Uuid,
        name: Option<String>,
        mount_path: Option<String>,
        volume_uuid: Option<String>,
    ) -> Result<storage_root::Model, sea_orm::DbErr> {
        let existing = storage_root::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound(format!(
                "storage_root {} not found",
                id
            )))?;

        let mut active: storage_root::ActiveModel = existing.into();
        if let Some(name) = name {
            active.name = Set(name);
        }
        if let Some(mount_path) = mount_path {
            active.mount_path = Set(mount_path);
        }
        if let Some(volume_uuid) = volume_uuid {
            active.volume_uuid = Set(Some(volume_uuid));
        }
        active.update(&self.db).await
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), sea_orm::DbErr> {
        let existing = storage_root::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound(format!(
                "storage_root {} not found",
                id
            )))?;

        let active_model: storage_root::ActiveModel = existing.into();

        storage_root::Entity::delete(active_model)
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
