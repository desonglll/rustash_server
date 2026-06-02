use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

use crate::entity::file;
use crate::pagination::PaginationQuery;

#[derive(Clone)]
pub struct FileService {
    db: DatabaseConnection,
}

impl FileService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_list(
        &self,
        search: Option<String>,
        pagination: &PaginationQuery,
    ) -> Result<(Vec<file::Model>, u64), sea_orm::DbErr> {
        let mut select = file::Entity::find();

        if let Some(name) = search {
            select = select.filter(file::Column::Name.contains(&name));
        }

        let total = select
            .clone()
            .count(&self.db)
            .await?;

        let items = select
            .order_by_desc(file::Column::Id)
            .limit(pagination.per_page())
            .offset(pagination.offset())
            .all(&self.db)
            .await?;

        Ok((items, total))
    }

    pub async fn get_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<file::Model>, sea_orm::DbErr> {
        file::Entity::find_by_id(id)
            .one(&self.db)
            .await
    }

    pub async fn get_absolute_path(
        &self,
        id: Uuid,
    ) -> Result<Option<String>, sea_orm::DbErr> {
        let file = file::Entity::find_by_id(id)
            .one(&self.db)
            .await?;
        Ok(file.map(|f| f.path))
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<(), sea_orm::DbErr> {
        let existing = file::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound(format!(
                "file {} not found",
                id
            )))?;

        let active_model: file::ActiveModel = existing.into();
        file::Entity::delete(active_model)
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
