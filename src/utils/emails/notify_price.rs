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
            .register_template_string(
                "price_history_template",
                r#"<!DOCTYPE html>
                <html>
                <head>
                <style>
                .container { max-width: 600px; margin: auto; font-family: Arial, sans-serif; }
                .price-card { padding: 15px; border: 1px solid #ddd; border-radius: 8px; margin: 10px 0; }
                .current { background-color: #e3f2fd; }
                .history-item { padding: 10px; border-bottom: 1px solid #eee; }
                .price { font-weight: bold; color: #2196f3; }
                .highlight { color: #f44336; }
                </style>
                </head>
                <body>
                <div class="container">
                <h2>Price Information for {{product_name}}</h2>
                <div class="price-card current">
                <h3>Current Price: <span class="price">₹{{current_price}}</span></h3>
                <p>Highest Recorded: ₹{{highest_price}}</p>
                <p>Lowest Recorded: ₹{{lowest_price}}</p>
                </div>
                <h3>Price History</h3>
                {{#each price_history}}
                <div class="history-item">
                <span>{{this.date}}</span>
                <span class="price">₹{{this.price}}</span>
                </div>
                {{/each}}
                </div>
                </body>
                </html>"#,
            )
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
