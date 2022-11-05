use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DbRole::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DbRole::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DbRole::Name).string().not_null())
                    .col(ColumnDef::new(DbRole::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(DbRole::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DbRole::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum DbRole {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}
