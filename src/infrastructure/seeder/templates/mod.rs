pub(crate) mod cdp;
pub(crate) mod clean;

use std::path::Path;

use anyhow::{Context, Result};
use minijinja::Environment;

pub struct TemplateLoader {
    env: Environment<'static>,
}

impl TemplateLoader {
    pub fn new(base_path: &Path) -> Result<Self> {
        let mut env = Environment::new();
        env.set_loader(minijinja::path_loader(base_path));
        Ok(Self { env })
    }

    pub fn render(&self, template_name: &str, context: impl serde::Serialize) -> Result<String> {
        let template = self
            .env
            .get_template(template_name)
            .with_context(|| format!("failed to load template {}", template_name))?;
        template
            .render(context)
            .with_context(|| format!("failed to render template {}", template_name))
    }
}
