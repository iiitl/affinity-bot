use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use shuttle_runtime::SecretStore;

pub struct Email {
    to: String,
    email_body: String,
    smtp_host: String,
    smtp_username: String,
    smtp_password: String,
}

impl Email {
    pub fn new(
        to: String,
        email_body: String,
        secrets: &SecretStore,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let smtp_host = secrets
            .get("SMTP_HOST")
            .ok_or("SMTP_HOST not found in secrets")?;
        let smtp_username = secrets
            .get("SMTP_USERNAME")
            .ok_or("SMTP_USERNAME not found in secrets")?;
        let smtp_password = secrets
            .get("SMTP_PASSWORD")
            .ok_or("SMTP_PASSWORD not found in secrets")?;

        Ok(Email {
            to,
            email_body,
            smtp_host,
            smtp_username,
            smtp_password,
        })
    }

    fn new_transport(
        &self,
    ) -> Result<AsyncSmtpTransport<Tokio1Executor>, lettre::transport::smtp::Error> {
        let creds = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());
        let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.smtp_host)
            .unwrap()
            .credentials(creds)
            .build();
        Ok(transport)
    }

    pub async fn send_email(&self, subject: String) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from("no-reply@affinity.com".parse()?)
            .to(self.to.parse()?)
            .subject(subject)
            .header(lettre::message::header::ContentType::TEXT_HTML)
            .body(self.email_body.clone())?;
        
        let transport = self.new_transport()?;
        transport.send(email).await?;
        Ok(())
    }
}