use crate::prelude::Config;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::transport::smtp::Error;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

#[derive(Debug, Clone)]
pub(crate) struct SmtpClient {
    smtp_client: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpClient {
    pub fn init(config: &Config) -> Self {
        let credentials = Credentials::new(
            config.smtp.client_id.to_string(),
            config.smtp.client_password.to_string(),
        );

        let async_smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp.host)
            .expect(format!("should be able connect via SMTP to '{}'", config.smtp.host).as_str())
            .credentials(credentials)
            .build();

        SmtpClient {
            smtp_client: async_smtp_transport,
        }
    }

    pub async fn send(&self, message: Message) -> Result<Response, Error> {
        self.smtp_client.send(message).await
    }
}
