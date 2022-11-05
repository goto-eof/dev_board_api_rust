use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "db_role")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::db_role_user::Entity")]
    RoleUser,
    #[sea_orm(has_many = "super::db_role_permission::Entity")]
    RolePermission,
}

impl Related<super::db_role::Entity> for Entity {
    fn to() -> RelationDef {
        super::db_role_user::Relation::Role.def().rev()
    }

    fn via() -> Option<RelationDef> {
        Some(super::db_role_user::Relation::User.def())
    }
}

impl Related<super::db_permission::Entity> for Entity {
    fn to() -> RelationDef {
        super::db_role_user::Relation::Role.def().rev()
    }

    fn via() -> Option<RelationDef> {
        Some(super::db_role_permission::Relation::Permission.def())
    }
}

impl ActiveModelBehavior for ActiveModel {}
