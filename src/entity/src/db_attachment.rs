use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "db_message")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub hashcode: String,
    pub user_id: i32,
    pub item_id: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::db_item::Entity",
        from = "Column::ItemId",
        to = "super::db_item::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Item,
    #[sea_orm(
        belongs_to = "super::db_user::Entity",
        from = "Column::UserId",
        to = "super::db_user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::db_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Item.def()
    }
}

impl Related<super::db_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
