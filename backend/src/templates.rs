use actix_web::web;
use derive_more::Display;
pub use tera::{Context, Tera};

use crate::errors;

pub type Engine = Tera;
pub type Extractor = web::Data<Engine>;

pub fn build_template_engine(templates_dir: &str) -> Result<Engine, tera::Error> {
    // instantiating Tera this way ensures that we can guarantee all the templates are
    // loaded succesfully as soon as the app starts
    let mut engine = Tera::default();
    for template in Template::all_variants() {
        let template_path = format!("{templates_dir}/{template}");
        engine.add_template_file(template_path, Some(&template.to_string()))?;
    }

    Ok(engine)
}

#[derive(Display)]
pub enum Template {
    #[display(fmt = "base.html")]
    Base,
    #[display(fmt = "alert.html")]
    Alert,
    #[display(fmt = "verify.html")]
    VerifyUser,

    // beware: this variant *must* remain the last one to be declared
    __COUNT,
}

impl Template {
    const fn all_variants() -> [Self; Self::__COUNT as usize] {
        // beware: parent templates (i.e. base.html) must appear before child templates
        [Template::Base, Template::Alert, Template::VerifyUser]
    }

    pub fn render(
        &self,
        context: &Context,
        template_engine: &Engine,
    ) -> Result<String, errors::ApiError> {
        template_engine
            .render(&self.to_string(), context)
            .map_err(|e| errors::ApiError::Template {
                context: format!("Error rendering '{}' template: {}", self, e),
            })
    }
}
