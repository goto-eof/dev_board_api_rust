pub use sea_orm_migration::prelude::*;

mod m20221026_1_ct_db_user;
mod m20221027_1_ct_db_column;
mod m20221027_2_ct_db_item;
mod m20221105_2_create_table_db_role;
mod m20221105_2_ct_db_permission;
mod m20221105_4_ct_db_user_role;
mod m20221105_5_ct_db_role_permission;
mod m20221119_1_ct_db_board;
mod m20221119_2_ct_db_board_column;
mod m20221119_3_ct_db_board_user;
mod m20221221_2_ct_db_message;
mod m20221227_2_ct_db_attachment;
pub struct Migrator;
// sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/postgres -o entity/src
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221026_1_ct_db_user::Migration),
            Box::new(m20221027_1_ct_db_column::Migration),
            Box::new(m20221027_2_ct_db_item::Migration),
            Box::new(m20221105_2_ct_db_permission::Migration),
            Box::new(m20221105_2_create_table_db_role::Migration),
            Box::new(m20221105_4_ct_db_user_role::Migration),
            Box::new(m20221105_5_ct_db_role_permission::Migration),
            Box::new(m20221119_1_ct_db_board::Migration),
            Box::new(m20221119_2_ct_db_board_column::Migration),
            Box::new(m20221119_3_ct_db_board_user::Migration),
            Box::new(m20221221_2_ct_db_message::Migration),
            Box::new(m20221227_2_ct_db_attachment::Migration),
        ]
    }
}
