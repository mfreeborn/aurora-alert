use actix_web::web;
use derive_more::Display;
pub use tera::{Context, Tera};

use crate::errors;

pub type Engine = Tera;
pub type Extractor = web::Data<Engine>;

pub fn build_template_engine(template_directory: &str) -> Result<Engine, tera::Error> {
    Tera::new(template_directory)
}

#[derive(Display)]
pub enum Template {
    #[display(fmt = "alert.html")]
    Alert,
    #[display(fmt = "verify.html")]
    VerifyUser,
}

impl Template {
    pub fn render(
        &self,
        context: &Context,
        template_engine: &Engine,
    ) -> Result<String, errors::ApiError> {
        Ok(template_engine
            .render(&self.to_string(), context)
            .map_err(|e| errors::ApiError::Template {
                context: format!("Error rendering '{}' template: {}", self, e),
            })?)
    }
}
