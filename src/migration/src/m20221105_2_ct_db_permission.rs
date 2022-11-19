use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DbPermission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DbPermission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DbPermission::Name).string().not_null())
                    .col(ColumnDef::new(DbPermission::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(DbPermission::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DbPermission::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum DbPermission {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}
