use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;
use crate::{entity::file, handler::file_handler::FileQuery};

#[derive(Clone)]
pub struct FileService{
    db: DatabaseConnection,
}

impl FileService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
    pub async fn get_list(&self, req_query: FileQuery) -> Result<Vec<file::Model>, DbErr> {
        let mut select = file::Entity::find();

        if let Some(name) = req_query.search {
            select = select.filter(file::Column::Name.contains(&name));
        }

        select = select.order_by_desc(file::Column::Id);

        if let Some(l) = req_query.limit {
            select = select.limit(l);
        }

        if let Some(o) = req_query.offset {
            select = select.offset(o);
        }

        select.all(&self.db).await
    }


    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<file::Model>, DbErr> {
        file::Entity::find()
            .filter(file::Column::Id.eq(id))
            .one(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<(), DbErr> {
        if let Some(file_model) = file::Entity::find()
            .filter(file::Column::Id.eq(id))
            .one(&self.db)
            .await?
        {
            let active_model: file::ActiveModel = file_model.into();
            active_model.delete(&self.db).await?;
        }
        Ok(())
    }

    pub async fn get_absolute_path(&self, id: Uuid) -> Result<Option<String>, DbErr> {
        println!("uuid: {}", id);
        let file_opt = file::Entity::find()
            .filter(file::Column::Id.eq(id))
            .one(&self.db)
            .await?;

        Ok(file_opt.map(|f| f.path))
    }
}
