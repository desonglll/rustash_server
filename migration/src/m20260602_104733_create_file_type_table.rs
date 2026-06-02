use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table("file_type")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("name"))
                    .col(string("extensions"))
                    .to_owned(),
            )
            .await?;

        let stmt = Query::insert()
            .into_table(Alias::new("file_type"))
            .columns([Alias::new("name"), Alias::new("extensions")])
            .values_panic(["video".into(), "mp4,mkv,avi,mov".into()])
            .to_owned();

        manager.execute(stmt).await?;

        Ok(())



    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table("file_type").to_owned())
            .await
    }
}
