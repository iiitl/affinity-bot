use crate::scraper::myntra::scrape_product;
use anyhow::Context;
use entity::{notification_preferences, products};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::TransactionTrait;
use sea_orm::{prelude::Decimal, sqlx::types::chrono::Utc, DatabaseConnection, EntityTrait, Set};
use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue,
};
use tracing::error;
pub async fn myntra_add(
    options: &[ResolvedOption<'_>],
    db: &DatabaseConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    let product_id = match options.first() {
        Some(ResolvedOption {
            value: ResolvedValue::Number(id),
            ..
        }) => *id as i32,
        _ => return Ok("Please provide a valid ProductId".to_string()),
    };

    let email = match options.get(1) {
        Some(ResolvedOption {
            value: ResolvedValue::String(email),
            ..
        }) => email.to_string(),
        _ => return Ok("Please provide a valid email address".to_string()),
    };

    let time_interval = match options.get(2) {
        Some(ResolvedOption {
            value: ResolvedValue::Number(hours),
            ..
        }) => *hours as i32,
        _ => 24,
    };

    let price_threshold = match options.get(3) {
        Some(ResolvedOption {
            value: ResolvedValue::Number(threshold),
            ..
        }) => Decimal::try_from(*threshold).context("Invalid price threshold")?,
        _ => Decimal::new(0, 0),
    };

    let notify_on_lowest = match options.get(4) {
        Some(ResolvedOption {
            value: ResolvedValue::Boolean(notify),
            ..
        }) => *notify,
        _ => false,
    };

    let db = db.clone();

    tokio::spawn(async move {
        let result: Result<_, Box<dyn std::error::Error>> =
            async {
                let product_price = scrape_product(product_id.to_string().as_str()).await?;
                let txn = db.begin().await.context("Failed to start transaction")?;

                let existing_product = products::Entity::find()
                    .filter(products::Column::ProductId.eq(product_id))
                    .one(&txn)
                    .await
                    .context("Failed to check existing product")?;

                if existing_product.is_none() {
                    let products =
                        products::ActiveModel {
                            product_id: Set(product_id),
                            current_price: Set(Decimal::try_from(product_price)
                                .context("Invalid product price")?),
                            highest_price: Set(Decimal::try_from(product_price)
                                .context("Invalid product price")?),
                            lowest_price: Set(Decimal::try_from(product_price)
                                .context("Invalid product price")?),
                            last_updated: Set(Utc::now().naive_utc()),
                            ..Default::default()
                        };

                    products::Entity::insert(products)
                        .exec(&txn)
                        .await
                        .context("Failed to insert product")?;
                }

                let notification_preferences = notification_preferences::ActiveModel {
                    product_id: Set(product_id),
                    email: Set(email),
                    time_interval_hours: Set(time_interval),
                    price_threshold: Set(price_threshold),
                    notify_on_lowest: Set(notify_on_lowest),
                    notify_on_highest: Set(false),
                    last_notified: Set(Utc::now().naive_utc()),
                    created_at: Set(Utc::now().naive_utc()),
                    updated_at: Set(Utc::now().naive_utc()),
                    ..Default::default()
                };

                notification_preferences::Entity::insert(notification_preferences)
                    .exec(&txn)
                    .await
                    .context("Failed to insert notification preferences")?;

                txn.commit().await.context("Failed to commit transaction")?;
                Ok(())
            }
            .await;

        if let Err(e) = result {
            error!("Error in background task: {}", e);
        }
    });

    Ok("⏳ Your request is being processed. You will receive email notifications once setup is complete.".to_string())
}

pub fn register_add() -> CreateCommand {
    CreateCommand::new("myntra")
        .description("get notifications about the prices of products in Myntra")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Number,
                "productid",
                "Enter the ProductId",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "email", "Your email address")
                .required(true),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::Number,
            "timeintreval",
            "Time intrevals in hours",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::Number,
            "pricethreshold",
            "for custom price alerts",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::Boolean,
            "notifyonlowest",
            "get notification on lowest price",
        ))
}
