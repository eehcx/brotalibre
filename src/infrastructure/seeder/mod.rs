use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde_json::json;

pub(crate) mod commands;
pub(crate) mod templates;
pub(crate) mod ui_integration;

use crate::application::ports::AstroSeeder;
use crate::application::ports::Seeder;
use crate::domain::project::ArchitectureProfile;
use crate::domain::project::DocsEngine;
use crate::domain::project::PackageManager;
use crate::domain::project::ResolvedOptions;
use crate::domain::project::UiChoice;
use crate::domain::styles_choice::StylesChoice;
use crate::infrastructure::seeder::ui_integration::apply_ui_integration;

pub use self::commands::{CommandRunner, SystemCommandRunner};

pub struct SystemSeeder;

impl Seeder for SystemSeeder {
    fn ensure_required_tools(&self, package_manager: PackageManager) -> Result<()> {
        let mut runner = SystemCommandRunner;
        commands::ensure_required_tools(&mut runner, package_manager)
    }

    fn scaffold_angular_project(&self, project_name: &str, options: ResolvedOptions) -> Result<()> {
        let mut runner = SystemCommandRunner;
        commands::scaffold_angular_project(&mut runner, project_name, options)
    }

    fn apply_architecture_template(
        &self,
        project_dir: &Path,
        architecture: ArchitectureProfile,
        project_name: &str,
        styles: StylesChoice,
    ) -> Result<()> {
        let template_base = template_base()?;

        enable_strict_tsconfig(project_dir)?;

        match architecture {
            ArchitectureProfile::Clean => templates::clean::apply_clean_architecture_template(
                &template_base,
                project_dir,
                project_name,
                styles,
            ),
            ArchitectureProfile::Ddd => templates::ddd::apply_ddd_architecture_template(
                &template_base,
                project_dir,
                project_name,
                styles,
            ),
        }
    }

    fn apply_feature_template(
        &self,
        project_dir: &Path,
        architecture: ArchitectureProfile,
        name: &str,
        prefix: &str,
        fields: &[String],
        ui: UiChoice,
        styles: StylesChoice,
    ) -> Result<()> {
        let template_base = template_base()?;

        let fields_json: Vec<serde_json::Value> = fields
            .iter()
            .map(|f| {
                let parts: Vec<&str> = f.splitn(2, ':').collect();
                serde_json::json!({
                    "name": parts[0],
                    "type": parts.get(1).copied().unwrap_or("string"),
                    "required": true,
                })
            })
            .collect();

        match architecture {
            ArchitectureProfile::Clean => templates::clean::apply_clean_feature_template(
                &template_base,
                &project_dir.join("src/app"),
                name,
                prefix,
                &fields_json,
                ui,
                styles,
            ),
            ArchitectureProfile::Ddd => templates::ddd::apply_ddd_feature_template(
                &template_base,
                &project_dir.join("src/app/features"),
                name,
                prefix,
                &fields_json,
                ui,
                styles,
            ),
        }?;

        add_ngrx_deps_to_package_json(project_dir)?;
        if ui != UiChoice::None || styles == StylesChoice::TailwindCSS {
            patch_app_config(project_dir, architecture, name, ui)?;
            patch_app_routes(project_dir, architecture, name)?;
        }

        Ok(())
    }

    fn apply_ui_integration(
        &self,
        project_dir: &Path,
        ui: UiChoice,
        package_manager: PackageManager,
    ) -> Result<()> {
        let mut runner = SystemCommandRunner;
        apply_ui_integration(&mut runner, project_dir, ui, package_manager)
    }
}

impl AstroSeeder for SystemSeeder {
    fn ensure_astro_tools(&self, _package_manager: PackageManager) -> Result<()> {
        // TODO(commit 3): check node + npm/npx for `npm create astro`.
        Ok(())
    }

    fn scaffold_astro_project(
        &self,
        _project_name: &str,
        _docs_engine: DocsEngine,
        _package_manager: PackageManager,
    ) -> Result<()> {
        // TODO(commit 3): shell out to `npm create starlight@latest` / `npm create astro@latest`.
        bail!("Astro scaffolding is not implemented yet")
    }

    fn apply_astro_template(
        &self,
        _project_dir: &Path,
        _docs_engine: DocsEngine,
        _project_name: &str,
        _locales: &[String],
    ) -> Result<()> {
        // TODO(commit 3): render per-locale .j2 templates from templates/astro.
        bail!("Astro template application is not implemented yet")
    }
}

fn template_base() -> Result<PathBuf> {
    let executable_dir = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf));
    let current_dir = std::env::current_dir().context("unable to resolve current directory")?;

    Ok(resolve_template_base(
        executable_dir.as_deref(),
        &current_dir,
    ))
}

fn resolve_template_base(executable_dir: Option<&Path>, current_dir: &Path) -> PathBuf {
    if let Some(executable_dir) = executable_dir {
        let bundled_templates = executable_dir.join("templates").join("angular");
        if bundled_templates.is_dir() {
            return bundled_templates;
        }
    }

    current_dir.join("templates").join("angular")
}

fn strip_json_comments(input: &str) -> String {
    let mut result = String::new();
    let mut in_block = false;
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if !in_block && c == '/' && chars.peek() == Some(&'*') {
            in_block = true;
            chars.next();
        } else if in_block && c == '*' && chars.peek() == Some(&'/') {
            in_block = false;
            chars.next();
        } else if !in_block {
            result.push(c);
        }
    }
    result
}

fn enable_strict_tsconfig(project_dir: &Path) -> Result<()> {
    let tsconfig_path = project_dir.join("tsconfig.json");
    if !tsconfig_path.exists() {
        return Ok(());
    }
    let content = std::fs::read_to_string(&tsconfig_path)?;
    let clean = strip_json_comments(&content);
    let mut json: serde_json::Value = serde_json::from_str(&clean).with_context(|| {
        format!(
            "failed to parse tsconfig at {}, first 100 chars: {:?}",
            tsconfig_path.display(),
            &clean[..clean.len().min(100)]
        )
    })?;
    if let Some(obj) = json
        .get_mut("compilerOptions")
        .and_then(serde_json::Value::as_object_mut)
    {
        obj.insert("strict".to_string(), serde_json::Value::Bool(true));
    }
    let out = serde_json::to_string_pretty(&json)?;
    std::fs::write(&tsconfig_path, &out)?;
    Ok(())
}

fn add_ngrx_deps_to_package_json(project_dir: &Path) -> Result<()> {
    let pkg_path = project_dir.join("package.json");
    if !pkg_path.exists() {
        return Ok(());
    }
    let content = std::fs::read_to_string(&pkg_path)?;
    let mut json: serde_json::Value = serde_json::from_str(&content)
        .with_context(|| format!("failed to parse package.json at {}", pkg_path.display()))?;

    if let Some(obj) = json
        .get_mut("dependencies")
        .and_then(serde_json::Value::as_object_mut)
    {
        obj.entry("@ngrx/signals".to_string())
            .or_insert(serde_json::Value::String("^21.0.0".to_string()));
        obj.entry("@ngrx/operators".to_string())
            .or_insert(serde_json::Value::String("^21.0.0".to_string()));
    }

    // Add overrides to resolve peer dep conflict until ngrx v22 stable
    let overrides = json!({
        "@ngrx/operators": { "@angular/core": "$@angular/core" },
        "@ngrx/signals": { "@angular/core": "$@angular/core" },
    });
    if json.get("overrides").is_none() {
        json["overrides"] = overrides;
    }

    std::fs::write(&pkg_path, serde_json::to_string_pretty(&json)?)?;
    Ok(())
}

fn patch_app_config(
    project_dir: &Path,
    architecture: ArchitectureProfile,
    name: &str,
    ui: UiChoice,
) -> Result<()> {
    let path = project_dir.join("src/app/app.config.ts");
    if !path.exists() {
        return Ok(());
    }
    let kebab = normalize_name(name);
    let snake = kebab.replace('-', "_").to_uppercase();
    let feature_path = match architecture {
        ArchitectureProfile::Clean => format!("./{kebab}"),
        ArchitectureProfile::Ddd => format!("./features/{kebab}"),
    };
    let provider_import = format!(
        "import {{ {snake}_PROVIDERS }} from '{feature_path}/infrastructure/{kebab}.provider';"
    );
    let mut content = std::fs::read_to_string(&path)?;
    if !content.contains(&provider_import) {
        content = format!("{provider_import}\n{content}");
    }
    if !content.contains(&format!("...{snake}_PROVIDERS")) {
        content = content.replacen(
            "providers: [",
            &format!("providers: [\n    ...{snake}_PROVIDERS,"),
            1,
        );
    }
    if !content.contains("provideHttpClient") {
        content = format!(
            "import {{ provideHttpClient, withFetch }} from '@angular/common/http';\n{content}"
        );
        content = content.replacen(
            "providers: [",
            "providers: [\n    provideHttpClient(withFetch()),",
            1,
        );
    }
    if ui == UiChoice::Material && !content.contains("provideAnimations") {
        content = format!(
            "import {{ provideAnimations }} from '@angular/platform-browser/animations';\n{content}"
        );
        content = content.replacen("providers: [", "providers: [\n    provideAnimations(),", 1);
    }
    if content.contains("provideRouter(routes)") {
        content = content.replace(
            "provideRouter(routes)",
            "provideRouter(routes, withComponentInputBinding())",
        );
        content =
            format!("import {{ withComponentInputBinding }} from '@angular/router';\n{content}");
    }
    std::fs::write(path, content)?;
    Ok(())
}

fn patch_app_routes(
    project_dir: &Path,
    architecture: ArchitectureProfile,
    name: &str,
) -> Result<()> {
    let path = project_dir.join("src/app/app.routes.ts");
    if !path.exists() {
        return Ok(());
    }
    let kebab = normalize_name(name);
    let pascal = pascal_case(&kebab);
    let feature_path = match architecture {
        ArchitectureProfile::Clean => format!("./{kebab}"),
        ArchitectureProfile::Ddd => format!("./features/{kebab}"),
    };
    let mut content = std::fs::read_to_string(&path)?;
    if content.contains(&format!("path: '{kebab}'")) {
        return Ok(());
    }
    let routes = format!(
        "  {{ path: '{kebab}', loadComponent: () => import('{feature_path}/presentation/list/{kebab}-list.component').then(m => m.{pascal}ListComponent) }},\n  {{ path: '{kebab}/new', loadComponent: () => import('{feature_path}/presentation/form/{kebab}-form.component').then(m => m.{pascal}FormComponent) }},\n  {{ path: '{kebab}/:id', loadComponent: () => import('{feature_path}/presentation/detail/{kebab}-detail.component').then(m => m.{pascal}DetailComponent) }},\n  {{ path: '{kebab}/:id/edit', loadComponent: () => import('{feature_path}/presentation/form/{kebab}-form.component').then(m => m.{pascal}FormComponent) }},\n"
    );
    if let Some(index) = content.rfind("];") {
        content.insert_str(index, &routes);
    } else {
        content = format!(
            "import {{ Routes }} from '@angular/router';\n\nexport const routes: Routes = [\n{routes}];\n"
        );
    }
    std::fs::write(path, content)?;
    Ok(())
}

fn normalize_name(name: &str) -> String {
    name.to_lowercase().replace(' ', "-")
}

fn pascal_case(name: &str) -> String {
    name.split('-')
        .map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .map(|first| first.to_uppercase().collect::<String>() + chars.as_str())
                .unwrap_or_default()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn template_base_prefers_templates_next_to_executable() {
        let tmp = tempdir().unwrap();
        let executable_dir = tmp.path().join("bin");
        let current_dir = tmp.path().join("workspace");
        let bundled_templates = executable_dir.join("templates/angular");

        fs::create_dir_all(&bundled_templates).unwrap();
        fs::create_dir_all(current_dir.join("templates/angular")).unwrap();

        assert_eq!(
            resolve_template_base(Some(&executable_dir), &current_dir),
            bundled_templates
        );
    }

    #[test]
    fn template_base_falls_back_to_current_directory() {
        let tmp = tempdir().unwrap();
        let executable_dir = tmp.path().join("bin");
        let current_dir = tmp.path().join("workspace");
        let workspace_templates = current_dir.join("templates/angular");

        fs::create_dir_all(&workspace_templates).unwrap();

        assert_eq!(
            resolve_template_base(Some(&executable_dir), &current_dir),
            workspace_templates
        );
    }

    #[test]
    fn template_loader_uses_embedded_templates_when_filesystem_templates_are_missing() {
        let tmp = tempdir().unwrap();
        let missing_templates = tmp.path().join("missing-templates");
        let loader = templates::TemplateLoader::new(&missing_templates).unwrap();

        let rendered = loader.render("app/app.config.ts.j2", ()).unwrap();
        assert!(rendered.contains("ApplicationConfig"));
    }

    #[test]
    fn strip_json_comments_removes_block_comments() {
        let input = r#"{
  /* comment */
  "key": "value" /* another */
}"#;
        let result = strip_json_comments(input);
        assert!(!result.contains("/* comment */"));
        assert!(!result.contains("/* another */"));
        assert!(result.contains("\"key\": \"value\""));
    }

    #[test]
    fn strip_json_comments_preserves_non_comment_slashes() {
        let input = r#"{"url": "http://example.com"}"#;
        let result = strip_json_comments(input);
        assert_eq!(result, input);
    }

    #[test]
    fn strip_json_comments_handles_nested_block_comment() {
        let input = r#"{"a": 1 /* outer /* inner */ end */, "b": 2}"#;
        let result = strip_json_comments(input);
        // Simple block comment handling: once in /* we only look for */
        assert!(!result.contains("/* outer"));
        assert!(result.contains("\"a\": 1"));
        assert!(result.contains("\"b\": 2"));
    }

    #[test]
    fn enable_strict_tsconfig_adds_strict_true() {
        let tmp = tempdir().unwrap();
        let ts = tmp.path().join("tsconfig.json");
        fs::write(&ts, r#"{"compilerOptions": {"target": "ES2022"}}"#).unwrap();

        enable_strict_tsconfig(tmp.path()).unwrap();

        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&ts).unwrap()).unwrap();
        assert_eq!(
            content["compilerOptions"]["strict"],
            serde_json::Value::Bool(true)
        );
    }

    #[test]
    fn enable_strict_tsconfig_handles_comments() {
        let tmp = tempdir().unwrap();
        let ts = tmp.path().join("tsconfig.json");
        fs::write(
            &ts,
            r#"{"compilerOptions": /* comment */ {"target": "ES2022"}}"#,
        )
        .unwrap();

        enable_strict_tsconfig(tmp.path()).unwrap();

        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&ts).unwrap()).unwrap();
        assert_eq!(
            content["compilerOptions"]["strict"],
            serde_json::Value::Bool(true)
        );
    }

    #[test]
    fn enable_strict_tsconfig_does_nothing_when_no_tsconfig() {
        let tmp = tempdir().unwrap();
        let result = enable_strict_tsconfig(tmp.path());
        assert!(result.is_ok());
    }

    #[test]
    fn add_ngrx_deps_adds_packages() {
        let tmp = tempdir().unwrap();
        let pkg = tmp.path().join("package.json");
        fs::write(&pkg, r#"{"dependencies": {"@angular/core": "^22.0.0"}}"#).unwrap();

        add_ngrx_deps_to_package_json(tmp.path()).unwrap();

        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&pkg).unwrap()).unwrap();
        assert_eq!(content["dependencies"]["@ngrx/signals"], "^21.0.0");
        assert_eq!(content["dependencies"]["@ngrx/operators"], "^21.0.0");
    }

    #[test]
    fn add_ngrx_deps_adds_overrides() {
        let tmp = tempdir().unwrap();
        let pkg = tmp.path().join("package.json");
        fs::write(&pkg, r#"{"dependencies": {"@angular/core": "^22.0.0"}}"#).unwrap();

        add_ngrx_deps_to_package_json(tmp.path()).unwrap();

        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&pkg).unwrap()).unwrap();
        let overrides = content["overrides"].as_object().unwrap();
        assert!(overrides.contains_key("@ngrx/signals"));
        assert!(overrides.contains_key("@ngrx/operators"));
    }

    #[test]
    fn add_ngrx_deps_does_not_duplicate_existing_deps() {
        let tmp = tempdir().unwrap();
        let pkg = tmp.path().join("package.json");
        fs::write(
            &pkg,
            r#"{"dependencies": {"@angular/core": "^22.0.0", "@ngrx/signals": "^19.0.0"}}"#,
        )
        .unwrap();

        add_ngrx_deps_to_package_json(tmp.path()).unwrap();

        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&pkg).unwrap()).unwrap();
        assert_eq!(content["dependencies"]["@ngrx/signals"], "^19.0.0");
    }

    #[test]
    fn add_ngrx_deps_skips_missing_package_json() {
        let tmp = tempdir().unwrap();
        let result = add_ngrx_deps_to_package_json(tmp.path());
        assert!(result.is_ok());
    }

    #[test]
    fn parse_feature_fields_simple() {
        let fields = ["name:string".to_string()];
        let result: Vec<serde_json::Value> = fields
            .iter()
            .map(|f| {
                let parts: Vec<&str> = f.splitn(2, ':').collect();
                serde_json::json!({"name": parts[0], "type": parts.get(1).copied().unwrap_or("string")})
            })
            .collect();
        assert_eq!(result[0]["name"], "name");
        assert_eq!(result[0]["type"], "string");
    }

    #[test]
    fn parse_feature_fields_defaults_type_to_string() {
        let fields = ["email".to_string()];
        let result: Vec<serde_json::Value> = fields
            .iter()
            .map(|f| {
                let parts: Vec<&str> = f.splitn(2, ':').collect();
                serde_json::json!({"name": parts[0], "type": parts.get(1).copied().unwrap_or("string")})
            })
            .collect();
        assert_eq!(result[0]["name"], "email");
        assert_eq!(result[0]["type"], "string");
    }

    #[test]
    fn parse_feature_fields_multiple_fields() {
        let fields = [
            "name:string".to_string(),
            "age:number".to_string(),
            "active:boolean".to_string(),
        ];
        let result: Vec<serde_json::Value> = fields
            .iter()
            .map(|f| {
                let parts: Vec<&str> = f.splitn(2, ':').collect();
                serde_json::json!({"name": parts[0], "type": parts.get(1).copied().unwrap_or("string")})
            })
            .collect();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0]["name"], "name");
        assert_eq!(result[2]["type"], "boolean");
    }

    #[test]
    fn parse_feature_empty_fields() {
        let fields: Vec<String> = vec![];
        let result: Vec<serde_json::Value> = fields
            .iter()
            .map(|f| {
                let parts: Vec<&str> = f.splitn(2, ':').collect();
                serde_json::json!({"name": parts[0], "type": parts.get(1).copied().unwrap_or("string")})
            })
            .collect();
        assert!(result.is_empty());
    }
}
