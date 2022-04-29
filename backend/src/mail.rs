use actix_web::{rt as actix_rt, web};
use lettre::{
    message::header, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

use crate::db;
use crate::errors;
use crate::templates;
use crate::types;

pub type Transport = AsyncSmtpTransport<Tokio1Executor>;
pub type Extractor = web::Data<Transport>;

pub fn build_mailer(username: &str, password: &str) -> anyhow::Result<Transport> {
    let creds = Credentials::new(username.to_owned(), password.to_owned());
    let mailer = Transport::relay("smtp.gmail.com")?
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
        user: &db::UserWithLocationsModel,
        alert_level: &types::AlertLevel,
    ) -> Result<RenderableEmailBuilder, errors::ApiError> {
        let mut context =
            templates::Context::from_serialize(user).map_err(|e| errors::ApiError::Template {
                context: format!(
                    "Error creating context for '{}' template: {}",
                    &self.template, e
                ),
            })?;
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

    pub fn add_context(
        self,
        user: &db::UserWithLocationsModel,
    ) -> Result<RenderableEmailBuilder, errors::ApiError> {
        let context =
            templates::Context::from_serialize(user).map_err(|e| errors::ApiError::Template {
                context: format!(
                    "Error creating context for '{}' template: {}",
                    &self.template, e
                ),
            })?;

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

    pub fn add_context(
        self,
        user: &db::UserWithLocationsModel,
    ) -> Result<RenderableEmailBuilder, errors::ApiError> {
        let context =
            templates::Context::from_serialize(user).map_err(|e| errors::ApiError::Template {
                context: format!(
                    "Error creating context for '{}' template: {}",
                    &self.template, e
                ),
            })?;

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
        template_engine: &templates::Engine,
    ) -> Result<RenderedEmailBuilder, errors::ApiError> {
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
    fn build(&self) -> anyhow::Result<Message> {
        let email = Message::builder()
            .header(header::ContentType::parse("text/html; charset=utf8")?)
            .from("Aurora Alert <aurora.alert.app@gmail.com>".parse()?)
            .to(self.to_address.parse()?)
            .subject(self.subject.clone())
            .body(self.body.clone())?;

        Ok(email)
    }

    pub fn build_email(self) -> Result<SendableEmail, errors::ApiError> {
        let email = self.build().map_err(|e| errors::ApiError::Email {
            context: format!("Error building email {}", e),
        })?;

        Ok(SendableEmail { email })
    }
}

pub struct SendableEmail {
    email: Message,
}

impl SendableEmail {
    pub fn send(self, mailer: Transport) {
        actix_rt::spawn(async move {
            match mailer.send(self.email.clone()).await {
                Ok(response) => log::info!("Email sent succesfully\n:{response:?}"),
                Err(e) => {
                    log::warn!(
                        "Error sending email to user: {}\nThe email message was:\n{:#?}",
                        e,
                        self.email
                    )
                }
            }
        });
    }
}

pub struct Email;

impl Email {
    pub fn new_alert(to_address: &str) -> AlertBuilder {
        AlertBuilder::new(to_address)
    }

    pub fn new_verify_user(to_address: &str) -> VerifyUserBuilder {
        VerifyUserBuilder::new(to_address)
    }

    pub fn new_user_already_registered(to_address: &str) -> UserAlreadyRegisteredBuilder {
        UserAlreadyRegisteredBuilder::new(to_address)
    }
}
