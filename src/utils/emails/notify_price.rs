use handlebars::Handlebars;
use sea_orm::prelude::DateTime;
use sea_orm::prelude::Decimal;

use crate::config::email::EmailConfig;

pub struct PriceHistoryEmail {
    product_name: String,
    current_price: Decimal,
    highest_price: Decimal,
    lowest_price: Decimal,
    price_history: Vec<(DateTime, Decimal)>,
    to: String,
}

impl PriceHistoryEmail {
    pub fn new(
        product_name: String,
        current_price: Decimal,
        highest_price: Decimal,
        lowest_price: Decimal,
        price_history: Vec<(DateTime, Decimal)>,
        to: String,
    ) -> Self {
        Self {
            product_name,
            current_price,
            highest_price,
            lowest_price,
            price_history,
            to,
        }
    }

    pub async fn send_price_history(&self) -> Result<(), String> {
        let mut handlebars = Handlebars::new();

        handlebars
            .register_template_file("price_history_template", "src/utils/hbs/price_history.hbs")
            .map_err(|e| e.to_string())?;

        let data = serde_json::json!({
            "product_name": self.product_name,
            "current_price": self.current_price,
            "highest_price": self.highest_price,
            "lowest_price": self.lowest_price,
            "price_history": self.price_history
                .iter()
                .map(|(date, price)| {
                    serde_json::json!({
                        "date": date.format("%Y-%m-%d %H:%M").to_string(),
                        "price": price
                    })
                })
                .collect::<Vec<_>>()
        });

        let email_body = handlebars
            .render("price_history_template", &data)
            .map_err(|e| e.to_string())?;

        EmailConfig::get()
            .send_email(
                self.to.clone(),
                "Price History Update".to_string(),
                email_body,
            )
            .await
            .map_err(|e| e.to_string())
    }
}
