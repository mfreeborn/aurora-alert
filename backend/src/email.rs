use axum::Extension as AxumExtension;
use lettre::{
    message::header, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

use crate::db;
use crate::templates;
use crate::types;
use crate::Result;

pub type EmailTransport = AsyncSmtpTransport<Tokio1Executor>;
pub type Extension = AxumExtension<EmailTransport>;

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

pub fn init(username: &str, password: &str) -> Result<EmailTransport, EmailError> {
    let creds = Credentials::new(username.to_string(), password.to_string());
    let mailer = EmailTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    Ok(mailer)
}

pub struct AlertBuilder {
    template: templates::Template,
    to_address: String,
}

impl AlertBuilder {
    fn new(to_address: &str) -> Self {
        Self {
            template: templates::Template::Alert,
            to_address: to_address.to_string(),
        }
    }

    pub fn add_context(
        self,
        user: &db::UserWithLocations,
        alert_level: &types::AlertLevel,
    ) -> Result<RenderableEmailBuilder> {
        let mut context = templates::Context::from_serialize(user)?;
        context.insert("alert_level", alert_level);

        Ok(RenderableEmailBuilder {
            template: self.template,
            to_address: self.to_address,
            subject: format!("Aurora alert level is now {alert_level}"),
            context,
        })
    }
}

pub struct VerifyUserBuilder {
    template: templates::Template,
    to_address: String,
    subject: String,
}

impl VerifyUserBuilder {
    fn new(to_address: &str) -> Self {
        Self {
            template: templates::Template::VerifyUser,
            to_address: to_address.to_string(),
            subject: String::from("Welcome to Aurora Alert"),
        }
    }

    pub fn add_context(self, user: &db::UserWithLocations) -> Result<RenderableEmailBuilder> {
        let context = templates::Context::from_serialize(user)?;

        Ok(RenderableEmailBuilder {
            template: self.template,
            to_address: self.to_address,
            subject: self.subject,
            context,
        })
    }
}

pub struct UserAlreadyRegisteredBuilder {
    template: templates::Template,
    to_address: String,
    subject: String,
}

impl UserAlreadyRegisteredBuilder {
    fn new(to_address: &str) -> Self {
        Self {
            template: templates::Template::UserAlreadyRegistered,
            to_address: to_address.to_string(),
            subject: String::from("Re-registering to Aurora Alert"),
        }
    }

    pub fn add_context(self, user: &db::UserWithLocations) -> Result<RenderableEmailBuilder> {
        let context = templates::Context::from_serialize(user)?;

        Ok(RenderableEmailBuilder {
            template: self.template,
            to_address: self.to_address,
            subject: self.subject,
            context,
        })
    }
}

pub struct RenderableEmailBuilder {
    template: templates::Template,
    to_address: String,
    subject: String,
    context: templates::Context,
}

impl RenderableEmailBuilder {
    pub fn render_body(
        self,
        template_engine: &templates::TemplateEngine,
    ) -> Result<RenderedEmailBuilder> {
        let body = self.template.render(&self.context, template_engine)?;

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

impl SendableEmail {
    pub fn send(self, mailer: EmailTransport) {
        tokio::spawn(async move {
            let recipient = self.email.envelope().to();
            match mailer.send(self.email.clone()).await {
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

/// Entry point for constructing an email which can be sent to the given address.
pub struct Email;

impl Email {
    /// Start constructing a new email for sending out an aurora alert.
    pub fn new_alert(to_address: &str) -> AlertBuilder {
        AlertBuilder::new(to_address)
    }

    /// Start constructing an email to verify a new user's identity.
    pub fn new_verify_user(to_address: &str) -> VerifyUserBuilder {
        VerifyUserBuilder::new(to_address)
    }

    /// Start constructing an email for a user who has tried to register an existing email address.
    pub fn new_user_already_registered(to_address: &str) -> UserAlreadyRegisteredBuilder {
        UserAlreadyRegisteredBuilder::new(to_address)
    }
}
