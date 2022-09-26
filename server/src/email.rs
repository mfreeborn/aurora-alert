use lettre::{
    message::header, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};
use tera::Context;
use tera::Tera;

use crate::common::AlertLevel;
use crate::configuration::EmailSettings;
use crate::db;
use crate::templates;
use crate::templates::Template;

pub type EmailTransport = AsyncSmtpTransport<Tokio1Executor>;

/// Coalesce all possible errors in this module into one type.
#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Lettre error")]
    Lettre(#[from] lettre::error::Error),

    #[error("Smtp error")]
    Smtp(#[from] lettre::transport::smtp::Error),

    #[error("Address error")]
    Address(#[from] lettre::address::AddressError),

    #[error("Content type error")]
    ContentType(#[from] lettre::message::header::ContentTypeErr),
}

pub struct AlertBuilder {
    template_engine: Tera,
    template: Template,
    to_address: String,
}

impl AlertBuilder {
    fn new(to_address: &str, template_engine: Tera) -> Self {
        let template = Template::Alert;
        Self {
            template_engine,
            template,
            to_address: to_address.to_string(),
        }
    }

    pub fn add_context(
        self,
        user: &db::UserWithLocations,
        alert_level: &AlertLevel,
    ) -> Result<RenderedEmailBuilder, anyhow::Error> {
        let mut context = Context::from_serialize(user)?;
        context.insert("alert_level", alert_level);

        let body = self.template.render(&context, &self.template_engine)?;

        Ok(RenderedEmailBuilder {
            to_address: self.to_address,
            subject: format!("Aurora alert level is now {alert_level}"),
            body,
        })
    }
}

pub struct VerifyUserBuilder {
    template_engine: Tera,
    template: Template,
    to_address: String,
    subject: String,
}

impl VerifyUserBuilder {
    fn new(to_address: &str, template_engine: Tera) -> Self {
        let template = Template::VerifyUser;
        Self {
            template_engine,
            template,
            to_address: to_address.to_string(),
            subject: String::from("Welcome to Aurora Alert"),
        }
    }

    pub fn add_context(
        self,
        user: &db::UserWithLocations,
    ) -> Result<RenderedEmailBuilder, anyhow::Error> {
        let context = templates::Context::from_serialize(user)?;

        let body = self.template.render(&context, &self.template_engine)?;

        Ok(RenderedEmailBuilder {
            to_address: self.to_address,
            subject: self.subject,
            body,
        })
    }
}

pub struct UserAlreadyRegisteredBuilder {
    template_engine: Tera,
    template: Template,
    to_address: String,
    subject: String,
}

impl UserAlreadyRegisteredBuilder {
    fn new(to_address: &str, template_engine: Tera) -> Self {
        let template = Template::UserAlreadyRegistered;

        Self {
            template_engine,
            template,
            to_address: to_address.to_string(),
            subject: String::from("Re-registering to Aurora Alert"),
        }
    }

    pub fn add_context(
        self,
        user: &db::UserWithLocations,
    ) -> Result<RenderedEmailBuilder, anyhow::Error> {
        let context = templates::Context::from_serialize(user)?;
        let body = self.template.render(&context, &self.template_engine)?;

        Ok(RenderedEmailBuilder {
            to_address: self.to_address,
            subject: self.subject,
            body,
        })
    }
}

pub struct RenderedEmailBuilder {
    to_address: String,
    subject: String,
    body: String,
}

impl RenderedEmailBuilder {
    fn build(&self) -> Result<Message, EmailError> {
        let email = Message::builder()
            .header(header::ContentType::parse("text/html; charset=utf8")?)
            .from("Aurora Alert <aurora.alert.app@gmail.com>".parse()?)
            .to(self.to_address.parse()?)
            .subject(self.subject.clone())
            .body(self.body.clone())?;

        Ok(email)
    }

    pub fn build_email(self) -> Result<SendableEmail, EmailError> {
        let email = self.build()?;

        Ok(SendableEmail { email })
    }
}

pub struct SendableEmail {
    email: Message,
}

/// Entry point for constructing an email which can be sent to the given
/// address.
#[derive(Clone, Debug)]
pub struct EmailClient {
    pub mailer: EmailTransport,
    pub template_engine: Tera,
}

impl EmailClient {
    pub fn new(config: &EmailSettings) -> Self {
        let creds = Credentials::new(config.username.to_string(), config.password.to_string());
        let mailer = EmailTransport::relay("smtp.gmail.com")
            .expect("failed to initialise gmail relay")
            .credentials(creds)
            .build();

        let template_engine = templates::init(&config.templates_dir)
            .expect("failed to initialise email template engine");

        Self {
            mailer,
            template_engine,
        }
    }

    /// Start constructing a new email for sending out an aurora alert.
    pub fn new_alert(&self, to_address: &str) -> AlertBuilder {
        let engine = self.template_engine.clone();
        AlertBuilder::new(to_address, engine)
    }

    /// Start constructing an email to verify a new user's identity.
    pub fn new_verify_user(&self, to_address: &str) -> VerifyUserBuilder {
        let engine = self.template_engine.clone();
        VerifyUserBuilder::new(to_address, engine)
    }

    /// Start constructing an email for a user who has tried to register an
    /// existing email address.
    pub fn new_user_already_registered(&self, to_address: &str) -> UserAlreadyRegisteredBuilder {
        let engine = self.template_engine.clone();
        UserAlreadyRegisteredBuilder::new(to_address, engine)
    }

    /// Send an email asynchronously.
    pub async fn send(&self, message: SendableEmail) {
        let mailer = self.mailer.clone();
        tokio::spawn(async move {
            let recipient = message.email.envelope().to();
            match mailer.send(message.email.clone()).await {
                Ok(_smtp_response) => {
                    tracing::debug!("email sent to {:?} successfully", recipient)
                }
                Err(e) => {
                    tracing::error!("error sending email to user: {}", e)
                }
            }
        });
    }
}
