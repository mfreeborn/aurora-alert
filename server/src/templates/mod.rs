use axum::Extension as AxumExtension;
use derive_more::Display;
pub use tera::{Context, Tera};

pub type TemplateEngine = Tera;
pub type Extension = AxumExtension<TemplateEngine>;

pub fn init(templates_dir: &str) -> Result<TemplateEngine, tera::Error> {
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
    #[display(fmt = "already_registered.html")]
    UserAlreadyRegistered,

    // Beware: this variant *must* remain the last one to be declared
    __COUNT,
}

impl Template {
    /// Compile time verification that, during module initialisation, all templates are added to the
    /// template engine.
    const fn all_variants() -> [Self; Self::__COUNT as usize] {
        // Beware: parent templates (i.e. base.html) must appear before child templates
        [
            Template::Base,
            Template::Alert,
            Template::VerifyUser,
            Template::UserAlreadyRegistered,
        ]
    }

    pub fn render(
        &self,
        context: &Context,
        template_engine: &TemplateEngine,
    ) -> Result<String, anyhow::Error> {
        Ok(template_engine.render(&self.to_string(), context)?)
    }
}
