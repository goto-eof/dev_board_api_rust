use entity::{db_board, db_board_column, db_column};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DbBoardColumn::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DbBoardColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DbBoardColumn::BoardId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bocol_boardid")
                            .from(db_board_column::Entity, db_board_column::Column::BoardId)
                            .to(db_board::Entity, db_board::Column::Id),
                    )
                    .col(ColumnDef::new(DbBoardColumn::ColumnId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bocol_columnid")
                            .from(db_board_column::Entity, db_board_column::Column::ColumnId)
                            .to(db_column::Entity, db_column::Column::Id),
                    )
                    .col(
                        ColumnDef::new(DbBoardColumn::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DbBoardColumn::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DbBoardColumn::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum DbBoardColumn {
    Table,
    Id,
    BoardId,
    ColumnId,
    CreatedAt,
    UpdatedAt,
}
