use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NotificationPreferences::Table)
                    .if_not_exists()
                    .col(pk_auto(NotificationPreferences::PreferenceId))
                    .col(integer(NotificationPreferences::ProductId).not_null())
                    .col(string(NotificationPreferences::Email).not_null())
                    .col(integer(NotificationPreferences::TimeIntervalHours).not_null())
                    .col(decimal(NotificationPreferences::PriceThreshold).not_null())
                    .col(
                        boolean(NotificationPreferences::NotifyOnLowest)
                            .not_null()
                            .default(false),
                    )
                    .col(
                        boolean(NotificationPreferences::NotifyOnHighest)
                            .not_null()
                            .default(false),
                    )
                    .col(timestamp(NotificationPreferences::LastNotified).not_null())
                    .col(timestamp(NotificationPreferences::CreatedAt).not_null())
                    .col(timestamp(NotificationPreferences::UpdatedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-notification_preferences-product_id")
                            .from(
                                NotificationPreferences::Table,
                                NotificationPreferences::ProductId,
                            )
                            .to(Products::Table, Products::ProductId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_notification_preferences_product_email")
                            .table(NotificationPreferences::Table)
                            .col(NotificationPreferences::ProductId)
                            .col(NotificationPreferences::Email)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notification_preferences_email")
                    .table(NotificationPreferences::Table)
                    .col(NotificationPreferences::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notification_preferences_last_notified")
                    .table(NotificationPreferences::Table)
                    .col(NotificationPreferences::LastNotified)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_notification_preferences_email")
                    .table(NotificationPreferences::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_notification_preferences_last_notified")
                    .table(NotificationPreferences::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(NotificationPreferences::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum NotificationPreferences {
    Table,
    PreferenceId,
    ProductId,
    Email,
    TimeIntervalHours,
    PriceThreshold,
    NotifyOnLowest,
    NotifyOnHighest,
    LastNotified,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Products {
    Table,
    ProductId,
}
