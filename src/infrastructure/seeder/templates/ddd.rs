use std::path::Path;

use anyhow::{Context, Result, bail};
use serde_json::json;

use super::TemplateLoader;
use crate::domain::styles_choice::StylesChoice;
use crate::infrastructure::seeder::commands::write_file;

pub(crate) fn apply_ddd_architecture_template(
    template_base: &Path,
    project_dir: &Path,
    project_name: &str,
    styles: StylesChoice,
) -> Result<()> {
    let app_dir = project_dir.join("src/app");
    if !app_dir.exists() {
        bail!(
            "could not find Angular app directory at `{}`",
            app_dir.display()
        );
    }

    patch_app_component_for_ddd(template_base, &app_dir, project_name, styles)?;
    patch_app_config_for_ddd(template_base, &app_dir)?;

    let features_dir = app_dir.join("features");
    std::fs::create_dir_all(&features_dir)
        .with_context(|| format!("failed to create {}", features_dir.display()))?;

    Ok(())
}

pub(crate) fn apply_ddd_feature_template(
    template_base: &Path,
    features_dir: &Path,
    name: &str,
    prefix: &str,
    fields: &[serde_json::Value],
) -> Result<()> {
    let loader = TemplateLoader::new(template_base)?;
    let name_kebab = name.to_string().to_lowercase().replace(' ', "-");
    let feature_dir = features_dir.join(&name_kebab);
    let name_pascal = name_kebab
        .split('-')
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>();
    let name_snake = name_kebab.replace('-', "_");

    let domain_dir = feature_dir.join("domain");
    let vo_dir = domain_dir.join("value-objects");
    let application_dir = feature_dir.join("application");
    let infra_dir = feature_dir.join("infrastructure");
    let mapper_dir = infra_dir.join("mappers");
    let dto_dir = infra_dir.join("dto");
    let presentation_dir = feature_dir.join("presentation");
    let list_dir = presentation_dir.join("list");
    let form_dir = presentation_dir.join("form");
    let detail_dir = presentation_dir.join("detail");

    for dir in [
        &domain_dir,
        &vo_dir,
        &application_dir,
        &infra_dir,
        &mapper_dir,
        &dto_dir,
        &list_dir,
        &form_dir,
        &detail_dir,
    ] {
        std::fs::create_dir_all(dir)
            .with_context(|| format!("failed to create directory {}", dir.display()))?;
    }

    let ctx = json!({
        "name": name_pascal,
        "name_kebab": name_kebab,
        "name_snake": name_snake,
        "prefix": prefix,
        "fields": fields,
    });

    write_file(
        &domain_dir.join(format!("{}.entity.ts", name_kebab)),
        &loader.render(
            "architecture/ddd/domain/{{ name }}.entity.ts.j2",
            ctx.clone(),
        )?,
    )?;
    write_file(
        &domain_dir.join(format!("{}-repository.port.ts", name_kebab)),
        &loader.render(
            "architecture/ddd/domain/{{ name }}-repository.port.ts.j2",
            ctx.clone(),
        )?,
    )?;
    write_file(
        &domain_dir.join(format!("{}.errors.ts", name_kebab)),
        &loader.render(
            "architecture/ddd/domain/{{ name }}.errors.ts.j2",
            ctx.clone(),
        )?,
    )?;
    write_file(
        &vo_dir.join(format!("{}-id.vo.ts", name_kebab)),
        &loader.render(
            "architecture/ddd/domain/value-objects/{{ name }}-id.vo.ts.j2",
            ctx.clone(),
        )?,
    )?;
    write_file(
        &application_dir.join(format!("{}.store.ts", name_kebab)),
        &loader.render("state/{{ name }}.store.ts.j2", ctx.clone())?,
    )?;

    for action in &["GetAll", "GetById", "Create", "Update", "Delete"] {
        let action_ctx = json!({
            "name": name_pascal,
            "name_kebab": name_kebab,
            "name_snake": name_snake,
            "action": action,
        });
        write_file(
            &application_dir.join(format!(
                "{}-{}.use-case.ts",
                action.to_lowercase(),
                name_kebab
            )),
            &loader.render(
                "architecture/ddd/application/{{ action }}.use-case.ts.j2",
                action_ctx,
            )?,
        )?;
    }

    write_file(
        &dto_dir.join(format!("{}.request.dto.ts", name_kebab)),
        &loader.render(
            "architecture/ddd/infrastructure/dto/{{ name }}.request.dto.ts.j2",
            ctx.clone(),
        )?,
    )?;
    write_file(
        &dto_dir.join(format!("{}.response.dto.ts", name_kebab)),
        &loader.render(
            "architecture/ddd/infrastructure/dto/{{ name }}.response.dto.ts.j2",
            ctx.clone(),
        )?,
    )?;
    write_file(
        &mapper_dir.join(format!("{}.mapper.ts", name_kebab)),
        &loader.render(
            "architecture/ddd/infrastructure/mappers/{{ name }}.mapper.ts.j2",
            ctx.clone(),
        )?,
    )?;
    write_file(
        &infra_dir.join(format!("{}.repository.ts", name_kebab)),
        &loader.render(
            "architecture/ddd/infrastructure/{{ name }}.repository.ts.j2",
            ctx.clone(),
        )?,
    )?;
    write_file(
        &infra_dir.join(format!("{}.provider.ts", name_kebab)),
        &loader.render(
            "architecture/ddd/infrastructure/{{ name }}.provider.ts.j2",
            ctx,
        )?,
    )?;

    Ok(())
}

pub(crate) fn patch_app_component_for_ddd(
    template_base: &Path,
    app_dir: &Path,
    project_name: &str,
    styles: StylesChoice,
) -> Result<()> {
    let loader = TemplateLoader::new(template_base)?;
    let style_url = format!("./app.{}", styles.file_extension());

    let (app_ts, app_html, template_url, component_class) = if app_dir.join("app.ts").exists() {
        (
            app_dir.join("app.ts"),
            app_dir.join("app.html"),
            "./app.html",
            "App",
        )
    } else {
        (
            app_dir.join("app.component.ts"),
            app_dir.join("app.component.html"),
            "./app.component.html",
            "AppComponent",
        )
    };

    let context = json!({
        "template_url": template_url,
        "style_url": style_url,
        "component_class": component_class,
        "project_name": project_name,
    });

    write_file(&app_ts, &loader.render("app/app.component.ts.j2", context)?)?;
    write_file(&app_html, &loader.render("app/app.component.html.j2", ())?)?;

    Ok(())
}

pub(crate) fn patch_app_config_for_ddd(template_base: &Path, app_dir: &Path) -> Result<()> {
    let loader = TemplateLoader::new(template_base)?;
    let app_config = app_dir.join("app.config.ts");
    write_file(&app_config, &loader.render("app/app.config.ts.j2", ())?)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    fn template_base() -> std::path::PathBuf {
        std::env::current_dir().unwrap().join("templates/angular")
    }

    fn create_app_dir(app_dir: &std::path::Path) {
        fs::create_dir_all(app_dir).unwrap();
        fs::write(app_dir.join("app.ts"), "").unwrap();
        fs::write(app_dir.join("app.html"), "").unwrap();
        fs::write(app_dir.join("app.config.ts"), "").unwrap();
    }

    fn feature_files(features_dir: &std::path::Path, name_kebab: &str) -> Vec<std::path::PathBuf> {
        let feature_dir = features_dir.join(name_kebab);
        vec![
            feature_dir
                .join("domain")
                .join(format!("{}.entity.ts", name_kebab)),
            feature_dir
                .join("domain")
                .join(format!("{}-repository.port.ts", name_kebab)),
            feature_dir
                .join("domain")
                .join(format!("{}.errors.ts", name_kebab)),
            feature_dir
                .join("domain/value-objects")
                .join(format!("{}-id.vo.ts", name_kebab)),
            feature_dir
                .join("application")
                .join(format!("{}.store.ts", name_kebab)),
            feature_dir
                .join("application")
                .join(format!("getall-{}.use-case.ts", name_kebab)),
            feature_dir
                .join("application")
                .join(format!("getbyid-{}.use-case.ts", name_kebab)),
            feature_dir
                .join("application")
                .join(format!("create-{}.use-case.ts", name_kebab)),
            feature_dir
                .join("application")
                .join(format!("update-{}.use-case.ts", name_kebab)),
            feature_dir
                .join("application")
                .join(format!("delete-{}.use-case.ts", name_kebab)),
            feature_dir
                .join("infrastructure/dto")
                .join(format!("{}.request.dto.ts", name_kebab)),
            feature_dir
                .join("infrastructure/dto")
                .join(format!("{}.response.dto.ts", name_kebab)),
            feature_dir
                .join("infrastructure/mappers")
                .join(format!("{}.mapper.ts", name_kebab)),
            feature_dir
                .join("infrastructure")
                .join(format!("{}.repository.ts", name_kebab)),
            feature_dir
                .join("infrastructure")
                .join(format!("{}.provider.ts", name_kebab)),
        ]
    }

    #[test]
    fn ddd_template_patches_app_files() {
        let tmp = tempdir().unwrap();
        let project_dir = tmp.path().join("demo");
        let app_dir = project_dir.join("src/app");
        create_app_dir(&app_dir);

        apply_ddd_architecture_template(
            &template_base(),
            &project_dir,
            "demo-app",
            StylesChoice::Css,
        )
        .unwrap();

        assert!(app_dir.join("app.ts").exists());
        assert!(app_dir.join("app.html").exists());
        assert!(app_dir.join("app.config.ts").exists());
        assert!(app_dir.join("features").exists());
    }

    #[test]
    fn ddd_template_uses_the_selected_style_file() {
        let tmp = tempdir().unwrap();
        let project_dir = tmp.path().join("demo");
        let app_dir = project_dir.join("src/app");
        create_app_dir(&app_dir);

        for (styles, extension) in [
            (StylesChoice::Css, "css"),
            (StylesChoice::Scss, "scss"),
            (StylesChoice::Sass, "sass"),
            (StylesChoice::Less, "less"),
            (StylesChoice::TailwindCSS, "css"),
        ] {
            apply_ddd_architecture_template(&template_base(), &project_dir, "demo-app", styles)
                .unwrap();

            let rendered = fs::read_to_string(app_dir.join("app.ts")).unwrap();
            assert!(rendered.contains(&format!("styleUrl: './app.{extension}'")));
        }
    }

    #[test]
    fn ddd_feature_creates_all_15_files() {
        let tmp = tempdir().unwrap();
        let project_dir = tmp.path().join("demo");
        let app_dir = project_dir.join("src/app");
        create_app_dir(&app_dir);
        apply_ddd_architecture_template(
            &template_base(),
            &project_dir,
            "demo-app",
            StylesChoice::Css,
        )
        .unwrap();

        let features_dir = app_dir.join("features");
        let fields = vec![
            serde_json::json!({"name": "email", "type": "string"}),
            serde_json::json!({"name": "age", "type": "number"}),
        ];
        apply_ddd_feature_template(&template_base(), &features_dir, "user", "api", &fields)
            .unwrap();

        let files = feature_files(&features_dir, "user");
        for f in &files {
            assert!(f.exists(), "missing file: {}", f.display());
        }
        assert_eq!(files.len(), 15);
    }

    #[test]
    fn ddd_feature_uses_kebab_pascal_snake_casing() {
        let tmp = tempdir().unwrap();
        let project_dir = tmp.path().join("demo");
        let app_dir = project_dir.join("src/app");
        create_app_dir(&app_dir);
        apply_ddd_architecture_template(
            &template_base(),
            &project_dir,
            "demo-app",
            StylesChoice::Css,
        )
        .unwrap();

        let features_dir = app_dir.join("features");
        apply_ddd_feature_template(&template_base(), &features_dir, "my-feature", "api", &[])
            .unwrap();

        let files = feature_files(&features_dir, "my-feature");
        for f in &files {
            assert!(f.exists(), "missing file: {}", f.display());
        }
    }

    #[test]
    fn ddd_feature_fails_when_app_dir_missing() {
        let tmp = tempdir().unwrap();

        let result = apply_ddd_architecture_template(
            &template_base(),
            &tmp.path().join("demo"),
            "demo-app",
            StylesChoice::Css,
        );
        assert!(result.is_err());
    }

    #[test]
    fn ddd_feature_with_multiple_word_name() {
        let tmp = tempdir().unwrap();
        let project_dir = tmp.path().join("demo");
        let app_dir = project_dir.join("src/app");
        create_app_dir(&app_dir);
        apply_ddd_architecture_template(
            &template_base(),
            &project_dir,
            "demo-app",
            StylesChoice::Css,
        )
        .unwrap();

        let features_dir = app_dir.join("features");
        apply_ddd_feature_template(
            &template_base(),
            &features_dir,
            "shopping cart",
            "api",
            &[serde_json::json!({"name": "items", "type": "string"})],
        )
        .unwrap();

        let files = feature_files(&features_dir, "shopping-cart");
        for f in &files {
            assert!(f.exists(), "missing file: {}", f.display());
        }
    }

    #[test]
    fn ddd_feature_uses_embedded_templates_without_template_dir() {
        let tmp = tempdir().unwrap();
        let project_dir = tmp.path().join("demo");
        let app_dir = project_dir.join("src/app");
        create_app_dir(&app_dir);
        apply_ddd_architecture_template(
            &template_base(),
            &project_dir,
            "demo-app",
            StylesChoice::Css,
        )
        .unwrap();

        let bad_base = tmp.path().join("no-templates");
        apply_ddd_feature_template(&bad_base, &app_dir.join("features"), "user", "api", &[])
            .unwrap();

        assert!(app_dir.join("features/user/domain/user.entity.ts").exists());
    }
}
