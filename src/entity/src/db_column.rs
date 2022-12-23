use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "db_column")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub order: i64,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::db_board_column::Entity")]
    DbBoardColumn,
    #[sea_orm(has_many = "super::db_item::Entity")]
    DbItem,
}

impl Related<super::db_board_column::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DbBoardColumn.def()
    }
}

impl Related<super::db_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DbItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
