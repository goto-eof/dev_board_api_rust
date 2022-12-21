use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "db_item")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub column_id: i32,
    pub assignee_id: Option<i32>,
    pub reporter_id: Option<i32>,
    pub name: String,
    pub environment: Option<String>,
    pub issue_type: Option<i32>,
    pub priority: i32,
    pub order: i64,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        from = "Column::ColumnId",
        belongs_to = "super::db_column::Entity",
        to = "super::db_column::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Column,

    #[sea_orm(
        from = "Column::AssigneeId",
        belongs_to = "super::db_user::Entity",
        to = "super::db_user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Assignee,
}

impl Related<super::db_column::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Column.def()
    }
}

impl Related<super::db_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Assignee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
