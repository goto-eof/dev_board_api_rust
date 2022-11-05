use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "db_role_permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub role_id: i32,
    pub permission_id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        from = "Column::RoleId",
        belongs_to = "super::db_role::Entity",
        to = "super::db_role::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Role,
    #[sea_orm(
        from = "Column::PermissionId",
        belongs_to = "super::db_permission::Entity",
        to = "super::db_permission::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Permission,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::db_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Related<super::db_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permission.def()
    }
}
