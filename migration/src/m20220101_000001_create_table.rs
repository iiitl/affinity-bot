use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(pk_auto(Products::ProductId))
                    .col(decimal(Products::CurrentPrice).not_null())
                    .col(decimal(Products::HighestPrice).not_null())
                    .col(decimal(Products::LowestPrice).not_null())
                    .col(timestamp(Products::LastUpdated).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Products::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Products {
    Table,
    ProductId,
    CurrentPrice,
    HighestPrice,
    LowestPrice,
    LastUpdated,
}
