use entity::{db_permission, db_role, db_role_permission};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DbRolePermission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DbRolePermission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DbRolePermission::PermissionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rp_permissionid")
                            .from(
                                db_role_permission::Entity,
                                db_role_permission::Column::PermissionId,
                            )
                            .to(db_permission::Entity, db_permission::Column::Id),
                    )
                    .col(
                        ColumnDef::new(DbRolePermission::RoleId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rp_roleid")
                            .from(
                                db_role_permission::Entity,
                                db_role_permission::Column::RoleId,
                            )
                            .to(db_role::Entity, db_role::Column::Id),
                    )
                    .col(
                        ColumnDef::new(DbRolePermission::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DbRolePermission::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DbRolePermission::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum DbRolePermission {
    Table,
    Id,
    Name,
    RoleId,
    PermissionId,
    CreatedAt,
    UpdatedAt,
}
