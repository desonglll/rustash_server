use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, Set};

use crate::entity::file_type;

#[derive(Clone)]
pub struct FileTypeService {
    db: DatabaseConnection,
}

impl FileTypeService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self) -> Result<Vec<file_type::Model>, sea_orm::DbErr> {
        file_type::Entity::find()
            .order_by_asc(file_type::Column::Id)
            .all(&self.db)
            .await
    }

    pub async fn get_by_id(
        &self,
        id: i64,
    ) -> Result<Option<file_type::Model>, sea_orm::DbErr> {
        file_type::Entity::find_by_id(id)
            .one(&self.db)
            .await
    }

    pub async fn create(
        &self,
        name: String,
        extensions: String,
    ) -> Result<file_type::Model, sea_orm::DbErr> {
        let model = file_type::ActiveModel {
            id: Set(0),
            name: Set(name),
            extensions: Set(extensions),
        };
        model.insert(&self.db).await
    }

    pub async fn update(
        &self,
        id: i64,
        name: Option<String>,
        extensions: Option<String>,
    ) -> Result<file_type::Model, sea_orm::DbErr> {
        let existing = file_type::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound(format!(
                "file_type {} not found",
                id
            )))?;

        let mut active: file_type::ActiveModel = existing.into();
        if let Some(name) = name {
            active.name = Set(name);
        }
        if let Some(extensions) = extensions {
            active.extensions = Set(extensions);
        }
        active.update(&self.db).await
    }

    pub async fn delete(&self, id: i64) -> Result<(), sea_orm::DbErr> {
        let existing = file_type::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound(format!(
                "file_type {} not found",
                id
            )))?;

        let active_model: file_type::ActiveModel = existing.into();

        file_type::Entity::delete(active_model)
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
