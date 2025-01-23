use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use once_cell::sync::OnceCell;
use shuttle_runtime::SecretStore;

static EMAIL_CONFIG: OnceCell<EmailConfig> = OnceCell::new();

pub struct EmailConfig {
    smtp_host: String,
    smtp_username: String,
    smtp_password: String,
}

impl EmailConfig {
    pub fn init(secrets: &SecretStore) -> Result<(), Box<dyn std::error::Error>> {
        let config = Self {
            smtp_host: secrets
                .get("SMTP_HOST")
                .ok_or("SMTP_HOST not found")?
                .to_string(),
            smtp_username: secrets
                .get("SMTP_USERNAME")
                .ok_or("SMTP_USERNAME not found")?
                .to_string(),
            smtp_password: secrets
                .get("SMTP_PASSWORD")
                .ok_or("SMTP_PASSWORD not found")?
                .to_string(),
        };
        EMAIL_CONFIG
            .set(config)
            .map_err(|_| "EmailConfig already initialized")?;
        Ok(())
    }

    pub fn get() -> &'static EmailConfig {
        EMAIL_CONFIG.get().expect("EmailConfig not initialized")
    }

    pub async fn send_email(
        &self,
        to: String,
        subject: String,
        body: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from("no-reply@affinity.com".parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(lettre::message::header::ContentType::TEXT_HTML)
            .body(body)?;

        let creds = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());
        let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.smtp_host)?
            .credentials(creds)
            .build();

        transport.send(email).await?;
        Ok(())
    }
}
