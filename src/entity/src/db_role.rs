//! SeaORM Entity. Generated by sea-orm-codegen 0.10.0
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "db_role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::db_user_role::Entity")]
    DbUserRole,
    #[sea_orm(has_many = "super::db_role_permission::Entity")]
    DbRolePermission,
}

impl Related<super::db_user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DbUserRole.def()
    }

    fn via() -> Option<RelationDef> {
        None
    }

    // fn find_related() -> Select<super::db_user_role::Entity> {
    //     Select::<super::db_user_role::Entity>::new().join_join_rev(
    //         sea_orm::JoinType::InnerJoin,
    //         Self::to(),
    //         Self::via(),
    //     )
    // }
}

impl Related<super::db_role_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DbRolePermission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
