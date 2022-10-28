pub use sea_orm_migration::prelude::*;

mod m20221027_000001_create_table_db_column;
mod m20221027_000001_create_table_db_item;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221027_000001_create_table_db_column::Migration),
            Box::new(m20221027_000001_create_table_db_item::Migration),
        ]
    }
}
