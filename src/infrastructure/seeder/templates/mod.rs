pub(crate) mod clean;
pub(crate) mod ddd;

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use include_dir::{Dir, include_dir};
use minijinja::Environment;
use serde_json::Value;

use crate::domain::project::UiChoice;
use crate::domain::styles_choice::StylesChoice;
use crate::infrastructure::seeder::commands::write_file;

static ANGULAR_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/angular");
static ASTRO_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/astro");

pub struct TemplateLoader {
    env: Environment<'static>,
}

impl TemplateLoader {
    pub fn new(base_path: &Path) -> Result<Self> {
        Self::with_embedded(base_path, &ANGULAR_TEMPLATES)
    }

    pub fn new_astro(base_path: &Path) -> Result<Self> {
        Self::with_embedded(base_path, &ASTRO_TEMPLATES)
    }

    fn with_embedded(base_path: &Path, embedded: &'static Dir<'static>) -> Result<Self> {
        let mut env = Environment::new();
        let filesystem_loader = base_path
            .is_dir()
            .then(|| minijinja::path_loader(base_path));
        env.set_loader(move |name| {
            if let Some(loader) = &filesystem_loader
                && let Some(source) = loader(name)?
            {
                return Ok(Some(source));
            }

            Ok(embedded
                .get_file(name)
                .and_then(|file| file.contents_utf8())
                .map(str::to_owned))
        });
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

pub(crate) fn patch_admin_shell(
    template_base: &Path,
    app_dir: &Path,
    project_name: &str,
) -> Result<()> {
    let loader = TemplateLoader::new(template_base)?;
    let layout_dir = app_dir.join("shared/layout");
    fs::create_dir_all(&layout_dir)
        .with_context(|| format!("failed to create {}", layout_dir.display()))?;

    let ctx = serde_json::json!({ "project_name": project_name });

    for file in [
        "admin-shell.component.ts",
        "admin-shell.component.html",
        "admin-shell.component.css",
    ] {
        write_file(
            &layout_dir.join(file),
            &loader.render(&format!("ui/_shared/layouts/{file}.j2"), &ctx)?,
        )?;
    }

    Ok(())
}

pub(crate) fn patch_app_routes(
    template_base: &Path,
    app_dir: &Path,
    project_name: &str,
) -> Result<()> {
    let loader = TemplateLoader::new(template_base)?;
    let pages_dir = app_dir.join("ui/_shared/pages");
    fs::create_dir_all(&pages_dir)
        .with_context(|| format!("failed to create {}", pages_dir.display()))?;

    let routes_path = app_dir.join("app.routes.ts");
    write_file(&routes_path, &loader.render("app/app.routes.ts.j2", ())?)?;

    for page in ["welcome", "dashboard", "not-found"] {
        let ctx = if page == "welcome" {
            serde_json::json!({ "project_name": project_name })
        } else {
            serde_json::json!({})
        };
        for ext in ["ts", "html", "css"] {
            write_file(
                &pages_dir.join(format!("{page}.component.{ext}")),
                &loader.render(&format!("ui/_shared/pages/{page}.component.{ext}.j2"), &ctx)?,
            )?;
        }
    }

    Ok(())
}

pub(crate) fn render_feature_presentation(
    loader: &TemplateLoader,
    feature_dir: &Path,
    name_kebab: &str,
    context: &Value,
    ui: UiChoice,
    styles: StylesChoice,
) -> Result<()> {
    if ui == UiChoice::None && styles != StylesChoice::TailwindCSS {
        return Ok(());
    }
    let ui_path = match ui {
        UiChoice::Material => "material",
        UiChoice::Primeng => "primeng",
        UiChoice::None => "tailwindcss",
    };
    for (view, suffix, has_css) in [
        ("list", "list", true),
        ("form", "form", true),
        ("detail", "detail", true),
    ] {
        let target_dir = feature_dir.join("presentation").join(view);
        write_file(
            &target_dir.join(format!("{name_kebab}-{suffix}.component.ts")),
            &loader.render(
                &format!("ui/{ui_path}/{view}-view/{{{{ name }}}}-{suffix}.component.ts.j2"),
                context,
            )?,
        )?;
        write_file(
            &target_dir.join(format!("{name_kebab}-{suffix}.component.html")),
            &loader.render(
                &format!("ui/{ui_path}/{view}-view/{{{{ name }}}}-{suffix}.component.html.j2"),
                context,
            )?,
        )?;
        if has_css {
            write_file(
                &target_dir.join(format!("{name_kebab}-{suffix}.component.css")),
                &loader.render(
                    &format!("ui/{ui_path}/{view}-view/{{{{ name }}}}-{suffix}.component.css.j2"),
                    context,
                )?,
            )?;
        }
        if view == "list" && ui == UiChoice::Material {
            let list_dir = target_dir;
            write_file(
                &list_dir.join("confirm-dialog.component.ts"),
                &loader.render(
                    "ui/material/list-view/confirm-dialog.component.ts.j2",
                    serde_json::json!({}),
                )?,
            )?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::TemplateLoader;

    #[test]
    fn falls_back_to_embedded_templates_when_a_filesystem_override_is_partial() {
        let tmp = tempdir().unwrap();
        let template_dir = tmp.path().join("templates");
        let overridden_template = template_dir.join("app/app.config.ts.j2");

        fs::create_dir_all(overridden_template.parent().unwrap()).unwrap();
        fs::write(&overridden_template, "override").unwrap();

        let loader = TemplateLoader::new(&template_dir).unwrap();

        assert_eq!(
            loader.render("app/app.config.ts.j2", ()).unwrap(),
            "override"
        );
        assert!(
            loader
                .render("app/app.component.ts.j2", ())
                .unwrap()
                .contains("Component")
        );
    }
}
