use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

use crate::error::ServerError;

pub struct Mail {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    username: String,
}

impl Mail {
    pub async fn new(username: String, password: String) -> crate::error::Result<Mail> {
        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .map_err(|_| ServerError::MailInit("Failed to init relay".to_string()))?
            .credentials(Credentials::new(username.clone(), password))
            .build();

        Ok(Mail {
            transport,
            username,
        })
    }

    pub async fn send_email(&self, receiver: &str, access_code: &str) -> crate::error::Result {
        let subject = "Password recovery";
        let body = format!("Password recovery access code: {}", access_code);

        let email = Message::builder()
            .from(
                self.username
                    .parse()
                    .map_err(|_| ServerError::MailSend("Failed to parse sender".to_string()))?,
            )
            .to(receiver
                .parse()
                .map_err(|_| ServerError::MailSend("Failed to parse receiver".to_string()))?)
            .subject(subject)
            .body(body.to_string())
            .map_err(|_| ServerError::MailSend("Failed to compose e-mail".to_string()))?;

        self.transport
            .send(email)
            .await
            .map_err(|_| ServerError::MailSend("Failed to send e-mail".to_string()))?;

        Ok(())
    }
}
