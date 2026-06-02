pub use sea_orm_migration::prelude::*;

mod m20260602_104733_create_file_type_table;
mod m20260602_104805_create_file_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260602_104733_create_file_type_table::Migration),
            Box::new(m20260602_104805_create_file_table::Migration),
        ]
    }
}
