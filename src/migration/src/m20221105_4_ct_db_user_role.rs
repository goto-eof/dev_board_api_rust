use entity::{db_role, db_user, db_user_role};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DbUserRole::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DbUserRole::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DbUserRole::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ur_userid")
                            .from(db_user_role::Entity, db_user_role::Column::UserId)
                            .to(db_user::Entity, db_user::Column::Id),
                    )
                    .col(ColumnDef::new(DbUserRole::RoleId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ur_roleid")
                            .from(db_user_role::Entity, db_user_role::Column::RoleId)
                            .to(db_role::Entity, db_role::Column::Id),
                    )
                    .col(ColumnDef::new(DbUserRole::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(DbUserRole::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DbUserRole::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum DbUserRole {
    Table,
    Id,
    UserId,
    RoleId,
    CreatedAt,
    UpdatedAt,
}
