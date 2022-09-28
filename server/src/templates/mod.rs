use derive_more::Display;
pub use tera::{Context, Tera};

pub type TemplateEngine = Tera;

pub fn init() -> Result<TemplateEngine, tera::Error> {
    // Instantiating Tera this way ensures that we can guarantee all the templates
    // are loaded succesfully as soon as the app starts
    let mut engine = Tera::default();
    engine
        .add_raw_templates(vec![
            ("base.html", include_str!("./base.html")),
            ("alert.html", include_str!("./alert.html")),
            ("verify.html", include_str!("./verify.html")),
            (
                "already_registered.html",
                include_str!("./already_registered.html"),
            ),
            ("base.html", include_str!("./base.html")),
        ])
        .expect("failed to load templates");

    Ok(engine)
}

#[derive(Display)]
pub enum Template {
    #[display(fmt = "alert.html")]
    Alert,
    #[display(fmt = "verify.html")]
    VerifyUser,
    #[display(fmt = "already_registered.html")]
    UserAlreadyRegistered,
}

impl Template {
    pub fn render(
        &self,
        context: &Context,
        template_engine: &TemplateEngine,
    ) -> Result<String, anyhow::Error> {
        Ok(template_engine.render(&self.to_string(), context)?)
    }
}
