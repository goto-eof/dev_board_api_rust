//! SeaORM Entity. Generated by sea-orm-codegen 0.10.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "db_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub itm_id: i32,
    pub itm_name: Option<String>,
    pub itm_type: Option<String>,
    pub itm_code: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub itm_description: Option<String>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
