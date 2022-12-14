use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "db_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::db_board_user::Entity")]
    DbBoardUser,
    #[sea_orm(has_many = "super::db_message::Entity")]
    DbMessage,
    #[sea_orm(has_many = "super::db_user_role::Entity")]
    DbUserRole,
}

impl Related<super::db_board_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DbBoardUser.def()
    }
}

impl Related<super::db_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DbMessage.def()
    }
}

impl Related<super::db_user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DbUserRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
