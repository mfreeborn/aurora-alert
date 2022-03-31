use crate::db;
use crate::errors;
use crate::templates;
use crate::types;

use actix_web::{rt as actix_rt, web};
use lettre::{
    message::header, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

pub type Transport = AsyncSmtpTransport<Tokio1Executor>;
pub type Extractor = web::Data<Transport>;

pub struct Email {
    pub message: Message,
}

impl Email {
    fn build_message(subject: &str, to_address: &str, body: String) -> anyhow::Result<Message> {
        let email = Message::builder()
            .header(header::ContentType::parse("text/html; charset=utf8")?)
            .from("Aurora Alert <aurora.alert.app@gmail.com>".parse()?)
            .to(to_address.parse()?)
            .subject(subject)
            .body(body)?;

        Ok(email)
    }

    pub fn build_verify(
        user: &db::UserWithLocationsModel,
        template_engine: &templates::Engine,
    ) -> Result<Self, errors::ApiError> {
        let template = templates::Template::Verify;
        let context =
            templates::Context::from_serialize(user).map_err(|e| errors::ApiError::Template {
                context: format!("Error creating context for '{}' template: {}", &template, e),
            })?;

        let body =
            template
                .render(&context, template_engine)
                .map_err(|e| errors::ApiError::Template {
                    context: format!("Error rendering '{}' template: {}", &template, e),
                })?;

        let message =
            Self::build_message("Welcome to Aurora Alert", &user.email, body).map_err(|e| {
                errors::ApiError::Email {
                    context: format!("Error building email: {}", e),
                }
            })?;

        Ok(Self { message })
    }

    pub fn build_alert(
        user: &db::UserWithLocationsModel,
        alert_level: &types::AlertLevel,
        template_engine: &templates::Engine,
    ) -> Result<Self, errors::ApiError> {
        let template = templates::Template::Alert;
        let mut context =
            templates::Context::from_serialize(user).map_err(|e| errors::ApiError::Template {
                context: format!("Error creating context for '{}' template: {}", &template, e),
            })?;
        context.insert("alert_level", alert_level);

        let body =
            template
                .render(&context, template_engine)
                .map_err(|e| errors::ApiError::Template {
                    context: format!("Error rendering '{}' template: {}", &template, e),
                })?;

        let message = Self::build_message(
            &format!("Aurora alert level is now {alert_level}"),
            &user.email,
            body,
        )
        .map_err(|e| errors::ApiError::Email {
            context: format!("Error building email: {}", e),
        })?;

        Ok(Self { message })
    }

    pub fn send(self, mailer: Transport) {
        actix_rt::spawn(async move {
            match mailer.send(self.message.clone()).await {
                Ok(response) => log::info!("Email sent succesfully\n:{response:?}"),
                Err(e) => {
                    log::warn!(
                        "Error sending email to user: {}\nThe email message was:\n{:#?}",
                        e,
                        self.message
                    )
                }
            }
        });
    }
}

pub fn build_mailer(username: &str, password: &str) -> anyhow::Result<Transport> {
    let creds = Credentials::new(username.to_owned(), password.to_owned());
    let mailer = Transport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    Ok(mailer)
}
