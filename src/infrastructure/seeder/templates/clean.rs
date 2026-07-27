use std::path::Path;

use anyhow::{bail, Result};
use serde_json::json;

use super::TemplateLoader;
use crate::infrastructure::seeder::commands::write_file;

pub(crate) fn apply_clean_architecture_template(
    template_base: &Path,
    project_dir: &Path,
    project_name: &str,
) -> Result<()> {
    let app_dir = project_dir.join("src/app");
    if !app_dir.exists() {
        bail!(
            "could not find Angular app directory at `{}`",
            app_dir.display()
        );
    }

    patch_app_component_for_clean(template_base, &app_dir, project_name)?;
    patch_app_config_for_clean(template_base, &app_dir)?;

    Ok(())
}

pub(crate) fn patch_app_component_for_clean(
    template_base: &Path,
    app_dir: &Path,
    project_name: &str,
) -> Result<()> {
    let loader = TemplateLoader::new(template_base)?;

    let (app_ts, app_html, template_url, style_url, component_class) =
        if app_dir.join("app.ts").exists() {
            (
                app_dir.join("app.ts"),
                app_dir.join("app.html"),
                "./app.html",
                "./app.scss",
                "App",
            )
        } else {
            (
                app_dir.join("app.component.ts"),
                app_dir.join("app.component.html"),
                "./app.component.html",
                "./app.scss",
                "AppComponent",
            )
        };

    let context = json!({
        "template_url": template_url,
        "style_url": style_url,
        "component_class": component_class,
        "project_name": project_name,
    });

    write_file(&app_ts, &loader.render("app.component.ts.j2", context)?)?;

    write_file(&app_html, &loader.render("app.component.html.j2", ())?)?;

    Ok(())
}

pub(crate) fn patch_app_config_for_clean(template_base: &Path, app_dir: &Path) -> Result<()> {
    let loader = TemplateLoader::new(template_base)?;
    let app_config = app_dir.join("app.config.ts");

    write_file(&app_config, &loader.render("app.config.ts.j2", ())?)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn clean_template_patches_app_files() {
        let tmp = tempdir().unwrap();
        let project_dir = tmp.path().join("demo");
        let app_dir = project_dir.join("src/app");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("app.ts"), "").unwrap();
        fs::write(app_dir.join("app.html"), "").unwrap();
        fs::write(app_dir.join("app.config.ts"), "").unwrap();

        let template_base = std::env::current_dir().unwrap().join("templates/angular");

        apply_clean_architecture_template(&template_base, &project_dir, "demo-app").unwrap();

        assert!(app_dir.join("app.ts").exists());
        assert!(app_dir.join("app.html").exists());
        assert!(app_dir.join("app.config.ts").exists());
    }
}
