use entity::{db_item, db_message, db_user};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DbAttachment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DbAttachment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DbAttachment::UserId).integer())
                    .col(ColumnDef::new(DbAttachment::Name).string())
                    .col(ColumnDef::new(DbAttachment::Hashcode).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_att_user_id")
                            .from(db_message::Entity, db_message::Column::UserId)
                            .to(db_user::Entity, db_user::Column::Id),
                    )
                    .col(ColumnDef::new(DbAttachment::ItemId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_att_item_id")
                            .from(db_message::Entity, db_message::Column::ItemId)
                            .to(db_item::Entity, db_item::Column::Id),
                    )
                    .col(
                        ColumnDef::new(DbAttachment::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DbAttachment::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DbAttachment::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum DbAttachment {
    Table,
    Id,
    Name,
    Hashcode,
    UserId,
    ItemId,
    Message,
    CreatedAt,
    UpdatedAt,
}
