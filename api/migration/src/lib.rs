pub use sea_orm_migration::prelude::*;

mod m20221213_102719_create_downloads_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20221213_102719_create_downloads_table::Migration)]
    }
}
