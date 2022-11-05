pub use sea_orm_migration::prelude::*;

mod m20221027_000001_create_table_db_column;
mod m20221027_000001_create_table_db_item;
mod m20221105_000001_create_table_db_user;
mod m20221105_000002_create_table_db_permission;
mod m20221105_000002_create_table_db_role;
mod m20221105_000004_create_table_db_user_role;
mod m20221105_000005_create_table_db_role_permission;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221027_000001_create_table_db_column::Migration),
            Box::new(m20221027_000001_create_table_db_item::Migration),
            Box::new(m20221105_000001_create_table_db_user::Migration),
            Box::new(m20221105_000002_create_table_db_permission::Migration),
            Box::new(m20221105_000002_create_table_db_role::Migration),
            Box::new(m20221105_000004_create_table_db_user_role::Migration),
            Box::new(m20221105_000005_create_table_db_role_permission::Migration),
        ]
    }
}
