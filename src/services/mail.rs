use std::{env, error::Error};

use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

pub struct Mail {
    pub subject: Option<String>,
    pub recipient: String,
    pub content: Option<String>,
}

impl Mail {
    pub fn send(self) -> Result<(), Box<dyn Error>> {
        let mail_user = env::var("MAIL_USER").unwrap();
        let mail_pass = env::var("MAIL_PASS").unwrap();
        let mail_server = env::var("MAIL_SERVER").unwrap();
        let mail_sender = env::var("MAIL_SENDER").unwrap();

        let email = Message::builder()
            .from(mail_sender.parse().unwrap())
            .to(format!("<{}>", self.recipient).parse().unwrap())
            .subject(self.subject.unwrap_or("".into()))
            .header(ContentType::TEXT_PLAIN)
            .body(self.content.unwrap_or("".into()))
            .unwrap();

        let creds = Credentials::new(mail_user, mail_pass);
        let mailer = SmtpTransport::relay(&mail_server)
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        if let Err(e) = mailer.send(&email) {
            tracing::error!("failed to send mail: {e:#?}");
            return Err(Box::new(e));
        }

        Ok(())
    }
}
