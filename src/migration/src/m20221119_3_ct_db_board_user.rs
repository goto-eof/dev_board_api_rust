use entity::{db_board, db_board_user, db_user};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DbBoardUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DbBoardUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DbBoardUser::BoardId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bous_boardid")
                            .from(db_board_user::Entity, db_board_user::Column::BoardId)
                            .to(db_board::Entity, db_board::Column::Id),
                    )
                    .col(ColumnDef::new(DbBoardUser::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bous_userid")
                            .from(db_board_user::Entity, db_board_user::Column::UserId)
                            .to(db_user::Entity, db_user::Column::Id),
                    )
                    .col(
                        ColumnDef::new(DbBoardUser::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DbBoardUser::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DbBoardUser::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum DbBoardUser {
    Table,
    Id,
    BoardId,
    UserId,
    CreatedAt,
    UpdatedAt,
}
