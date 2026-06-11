use axum::response::Html;
use tera::{Context, Tera};

use crate::{Error, app::AppConfig};

#[derive(Debug)]
pub struct RenderService {
    tera: Tera,
    templates_suffix: String,
}

impl RenderService {
    pub fn new(config: &AppConfig) -> Result<Self, Error> {
        let templates_path = config.templates_path.clone();
        let templates_suffix = config.templates_suffix.clone();
        Ok(Self {
            tera: Self::new_tera(templates_path, &templates_suffix)?,
            templates_suffix,
        })
    }

    fn new_tera(templates_path: String, templates_suffix: &str) -> Result<Tera, Error> {
        let glob = templates_path + "/**/*" + templates_suffix;
        let tera = Tera::new(&glob)?;

        Ok(tera)
    }

    pub fn render(&self, template_name: &str, context: &Context) -> Result<String, Error> {
        let template_name = template_name.to_string() + &self.templates_suffix;
        Ok(self.tera.render(&template_name, context)?)
    }

    pub fn render_html(
        &self,
        template_name: &str,
        context: &Context,
    ) -> Result<Html<String>, Error> {
        Ok(Html::from(self.render(template_name, context)?))
    }
}
