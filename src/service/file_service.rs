use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use uuid::Uuid;

use crate::entity::{file, storage_root};
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

        let total = select.clone().count(&self.db).await?;

        let items = select
            .order_by_desc(file::Column::Id)
            .limit(pagination.per_page())
            .offset(pagination.offset())
            .all(&self.db)
            .await?;

        Ok((items, total))
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<file::Model>, sea_orm::DbErr> {
        file::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn get_absolute_path(&self, id: Uuid) -> Result<Option<String>, sea_orm::DbErr> {
        let file = file::Entity::find_by_id(id).one(&self.db).await?;

        match file {
            Some(f) => {
                let root = storage_root::Entity::find_by_id(f.storage_root_id)
                    .one(&self.db)
                    .await?;
                match root {
                    Some(r) => Ok(Some(format!(
                        "{}/{}",
                        r.mount_path.trim_end_matches('/'),
                        f.path.trim_start_matches('/')
                    ))),
                    None => Ok(None),
                }
            }
            None => Ok(None),
        }
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<(), sea_orm::DbErr> {
        let existing = file::Entity::find_by_id(id).one(&self.db).await?.ok_or(
            sea_orm::DbErr::RecordNotFound(format!("file {} not found", id)),
        )?;

        let active_model: file::ActiveModel = existing.into();
        file::Entity::delete(active_model).exec(&self.db).await?;
        Ok(())
    }
}
