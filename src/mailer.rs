use anyhow::Result;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport, Transport};

pub struct Mailer {
    smtp_mailer: SmtpTransport,
    name: String,
    username: String,
}

impl Mailer {
    pub fn new(name: &str, username: &str, password: &str, relay: &str) -> Result<Self> {
        let smtp_mailer = SmtpTransport::relay(relay)?
            .credentials(Credentials::new(username.to_string(), password.to_string()))
            .build();

        Ok(Self {
            smtp_mailer,
            name: name.to_string(),
            username: username.to_string(),
        })
    }

    pub async fn notify(&self, email: &str, subject: &str, body: &str) -> Result<()> {
        let email = lettre::Message::builder()
            .from(format!("{} <{}>", self.name, self.username).parse()?)
            .to(format!("{} <{}>", " ", email).parse()?)
            .subject(subject)
            .body(body.to_string())?;

        match self.smtp_mailer.send(&email) {
            Ok(res) => println!("Email sent successfully: {:?}", res),
            Err(err) => println!("Email couldn't be sent: {:?}", err),
        }

        Ok(())
    }
}
