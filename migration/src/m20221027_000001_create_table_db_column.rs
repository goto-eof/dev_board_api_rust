use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DbColumn::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DbColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DbColumn::Name).string().not_null())
                    .col(ColumnDef::new(DbColumn::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(DbColumn::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(DbColumn::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum DbColumn {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}
