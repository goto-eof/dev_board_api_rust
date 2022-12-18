use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DbUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DbUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DbUser::Username).string().not_null())
                    .col(ColumnDef::new(DbUser::Password).string().not_null())
                    .col(ColumnDef::new(DbUser::Email).string().not_null())
                    .col(ColumnDef::new(DbUser::FirstName).string().not_null())
                    .col(ColumnDef::new(DbUser::LastName).string().not_null())
                    .col(ColumnDef::new(DbUser::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(DbUser::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DbUser::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum DbUser {
    Table,
    Id,
    Username,
    Password,
    Email,
    FirstName,
    LastName,
    CreatedAt,
    UpdatedAt,
}
