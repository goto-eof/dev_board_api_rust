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
                    .table(DbMessage::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DbMessage::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DbMessage::UserId).integer())
                    .col(ColumnDef::new(DbMessage::MessageType).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_msg_user_id")
                            .from(db_message::Entity, db_message::Column::UserId)
                            .to(db_user::Entity, db_user::Column::Id),
                    )
                    .col(ColumnDef::new(DbMessage::ItemId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_msg_item_id")
                            .from(db_message::Entity, db_message::Column::ItemId)
                            .to(db_item::Entity, db_item::Column::Id),
                    )
                    .col(ColumnDef::new(DbMessage::Message).string())
                    .col(ColumnDef::new(DbMessage::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(DbMessage::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DbMessage::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum DbMessage {
    Table,
    Id,
    MessageType,
    UserId,
    ItemId,
    Message,
    CreatedAt,
    UpdatedAt,
}
