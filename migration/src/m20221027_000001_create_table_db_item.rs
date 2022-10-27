use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DbItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DbItem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DbItem::Name).string().not_null())
                    .col(ColumnDef::new(DbItem::TType).string().not_null())
                    .col(ColumnDef::new(DbItem::Code).string().not_null())
                    .col(ColumnDef::new(DbItem::Description).string().not_null())
                    .col(ColumnDef::new(DbItem::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(DbItem::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(DbItem::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum DbItem {
    Table,
    Id,
    Name,
    TType,
    Code,
    Description,
    CreatedAt,
    UpdatedAt,
}
