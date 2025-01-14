use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PriceHistory::Table)
                    .if_not_exists()
                    .col(pk_auto(PriceHistory::HistoryId))
                    .col(integer(PriceHistory::ProductId).not_null())
                    .col(decimal(PriceHistory::Price).not_null())
                    .col(timestamp(PriceHistory::RecordedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-price_history-product_id")
                            .from(PriceHistory::Table, PriceHistory::ProductId)
                            .to(Products::Table, Products::ProductId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PriceHistory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PriceHistory {
    Table,
    HistoryId,
    ProductId,
    Price,
    RecordedAt,
}

// Reference to Products table for foreign key
#[derive(DeriveIden)]
enum Products {
    Table,
    ProductId,
}