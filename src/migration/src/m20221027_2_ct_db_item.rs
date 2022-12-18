use entity::{db_column, db_item, db_user};
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
                    .col(ColumnDef::new(DbItem::AssigneeId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_item_assegnee")
                            .from(db_item::Entity, db_item::Column::AssigneeId)
                            .to(db_user::Entity, db_user::Column::Id),
                    )
                    .col(ColumnDef::new(DbItem::Environment).string().not_null())
                    .col(ColumnDef::new(DbItem::Code).string().not_null())
                    .col(ColumnDef::new(DbItem::Description).string())
                    .col(ColumnDef::new(DbItem::Order).big_integer().not_null())
                    .col(ColumnDef::new(DbItem::Priority).integer())
                    .col(ColumnDef::new(DbItem::ColumnId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_item_column")
                            .from(db_item::Entity, db_item::Column::ColumnId)
                            .to(db_column::Entity, db_column::Column::Id),
                    )
                    .col(ColumnDef::new(DbItem::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(DbItem::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DbItem::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum DbItem {
    Table,
    Id,
    AssigneeId,
    ColumnId,
    Name,
    Order,
    Environment,
    Priority,
    Code,
    Description,
    CreatedAt,
    UpdatedAt,
}
