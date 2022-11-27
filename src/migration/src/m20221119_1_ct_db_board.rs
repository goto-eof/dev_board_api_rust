use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DbBoard::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DbBoard::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DbBoard::Name).string().not_null())
                    .col(ColumnDef::new(DbBoard::Description).string())
                    .col(ColumnDef::new(DbBoard::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(DbBoard::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DbBoard::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum DbBoard {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}
