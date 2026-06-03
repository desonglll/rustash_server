use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add storage_root_id column if it doesn't exist (was missing due to migration order bug)
        manager
            .alter_table(
                Table::alter()
                    .table("file")
                    .add_column(ColumnDef::new(Alias::new("storage_root_id")).text().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite doesn't support DROP COLUMN in older versions
        Ok(())
    }
}
