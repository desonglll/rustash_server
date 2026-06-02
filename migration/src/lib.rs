pub use sea_orm_migration::prelude::*;

mod m20260602_104733_create_file_type_table;
mod m20260602_104805_create_file_table;
mod m20260602_140058_create_storage_root_table;
mod m20260602_150000_add_storage_root_id_to_file;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260602_104733_create_file_type_table::Migration),
            Box::new(m20260602_140058_create_storage_root_table::Migration),
            Box::new(m20260602_104805_create_file_table::Migration),
            Box::new(m20260602_150000_add_storage_root_id_to_file::Migration),
        ]
    }
}
