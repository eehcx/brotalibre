#![allow(dead_code)]

use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};
use minijinja::Environment;
use serde_json::Value;

use crate::application::ports::Seeder;
use crate::domain::project::ArchitectureProfile;
use crate::domain::project::PackageManager;
use crate::domain::project::ResolvedOptions;
use crate::domain::project::UiChoice;
use crate::domain::styles_choice::StylesChoice;

pub struct TemplateContext {
    pub project_name: String,
    pub template_url: String,
    pub style_url: String,
    pub component_class: String,
}

pub struct TemplateLoader {
    env: Environment<'static>,
}

impl TemplateLoader {
    pub fn new() -> Result<Self> {
        let mut env = Environment::new();

        // Try to find templates relative to current exe first
        // Fall back to current working directory for development
        let templates_dir = if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let templates = exe_dir.join("templates").join("angular");
                if templates.exists() {
                    templates
                } else {
                    std::env::current_dir()?.join("templates").join("angular")
                }
            } else {
                std::env::current_dir()?.join("templates").join("angular")
            }
        } else {
            std::env::current_dir()?.join("templates").join("angular")
        };

        env.set_loader(minijinja::path_loader(templates_dir));
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

impl Default for TemplateLoader {
    fn default() -> Self {
        Self::new().expect("failed to initialize template loader")
    }
}

pub trait CommandRunner {
    fn run(&mut self, program: &str, args: &[String], cwd: Option<&Path>) -> Result<()>;
}

pub struct SystemCommandRunner;

impl CommandRunner for SystemCommandRunner {
    fn run(&mut self, program: &str, args: &[String], cwd: Option<&Path>) -> Result<()> {
        let mut cmd = Command::new(program);
        cmd.args(args);
        if let Some(path) = cwd {
            cmd.current_dir(path);
        }

        let status = cmd.status().with_context(|| {
            format!(
                "failed to start command `{}`",
                format_command(program, args)
            )
        })?;

        if !status.success() {
            bail!("command failed: {}", format_command(program, args));
        }

        Ok(())
    }
}

pub struct SystemSeeder;

impl Seeder for SystemSeeder {
    fn ensure_required_tools(&self, package_manager: PackageManager) -> Result<()> {
        let mut runner = SystemCommandRunner;
        ensure_required_tools(&mut runner, package_manager)
    }

    fn scaffold_angular_project(&self, project_name: &str, options: ResolvedOptions) -> Result<()> {
        let mut runner = SystemCommandRunner;
        scaffold_angular_project(&mut runner, project_name, options)
    }

    fn apply_architecture_template(
        &self,
        project_dir: &Path,
        architecture: ArchitectureProfile,
    ) -> Result<()> {
        apply_architecture_template(project_dir, architecture)
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

    fn apply_styles(
        &self,
        project_dir: &Path,
        styles: StylesChoice,
        package_manager: PackageManager,
    ) -> Result<()> {
        let mut runner = SystemCommandRunner;
        apply_styles(&mut runner, project_dir, styles, package_manager)
    }
}

fn ensure_required_tools(
    runner: &mut dyn CommandRunner,
    package_manager: PackageManager,
) -> Result<()> {
    for tool in ["node", "ng", package_manager_cli_name(package_manager)] {
        let args = vec!["--version".to_string()];
        runner
            .run(tool, &args, None)
            .with_context(|| format!("`{tool}` is required. Install it and retry."))?;
    }

    Ok(())
}

fn scaffold_angular_project(
    runner: &mut dyn CommandRunner,
    project_name: &str,
    options: ResolvedOptions,
) -> Result<()> {
    let package_manager = package_manager_cli_name(options.package_manager);

    let mut args = vec![
        "new".to_string(),
        project_name.to_string(),
        "--defaults".to_string(),
        "--standalone".to_string(),
        "--routing".to_string(),
        "--style=scss".to_string(),
        "--ssr=false".to_string(),
        format!("--package-manager={package_manager}"),
    ];

    if options.skip_install {
        args.push("--skip-install".to_string());
    }

    runner.run("ng", &args, None)
}

fn apply_architecture_template(
    project_dir: &Path,
    architecture: ArchitectureProfile,
) -> Result<()> {
    match architecture {
        ArchitectureProfile::Clean => apply_clean_architecture_template(project_dir),
        ArchitectureProfile::Cdp => apply_cdp_architecture_template(project_dir),
    }
}

fn apply_clean_architecture_template(project_dir: &Path) -> Result<()> {
    let app_dir = project_dir.join("src/app");
    if !app_dir.exists() {
        bail!(
            "could not find Angular app directory at `{}`",
            app_dir.display()
        );
    }

    let loader = TemplateLoader::new()?;

    write_file(
        &app_dir.join("domain/entities/greeting.entity.ts"),
        &loader.render("greeting.entity.ts.j2", ())?,
    )?;

    write_file(
        &app_dir.join("domain/ports/greeting-repository.port.ts"),
        &loader.render("greeting-repository.port.ts.j2", ())?,
    )?;

    write_file(
        &app_dir.join("application/use-cases/get-greeting.use-case.ts"),
        &loader.render("get-greeting.use-case.ts.j2", ())?,
    )?;

    write_file(
        &app_dir.join("infrastructure/adapters/static-greeting.repository.ts"),
        &loader.render("static-greeting.repository.ts.j2", ())?,
    )?;

    write_file(
        &app_dir.join("infrastructure/providers/greeting.provider.ts"),
        &loader.render("greeting.provider.ts.j2", ())?,
    )?;

    write_file(
        &app_dir.join("presentation/facades/home.facade.ts"),
        &loader.render("home.facade.ts.j2", ())?,
    )?;

    patch_app_component_for_clean(&app_dir)?;
    patch_app_config_for_clean(&app_dir)?;

    Ok(())
}

fn apply_cdp_architecture_template(project_dir: &Path) -> Result<()> {
    let app_dir = project_dir.join("src/app");
    if !app_dir.exists() {
        bail!(
            "could not find Angular app directory at `{}`",
            app_dir.display()
        );
    }

    write_file(
        &app_dir.join("core/models/health-status.model.ts"),
        r#"export interface HealthStatus {
  service: string;
  status: 'ok' | 'degraded';
  checkedAt: string;
}
"#,
    )?;

    write_file(
        &app_dir.join("core/environment/app-environment.ts"),
        r#"export const appEnvironment = {
  appName: 'ngseed-cdp-app',
  apiBaseUrl: '/api',
};
"#,
    )?;

    write_file(
        &app_dir.join("core/commons/logger.ts"),
        r#"export function logInfo(message: string): void {
  console.info(`[CDP] ${message}`);
}
"#,
    )?;

    write_file(
        &app_dir.join("core/auth/auth.types.ts"),
        r#"export interface AuthUser {
  id: string;
  role: string;
}
"#,
    )?;

    write_file(
        &app_dir.join("data/datasource/remote/health.datasource.ts"),
        r#"import { Injectable } from '@angular/core';
import { HealthStatus } from '../../../core/models/health-status.model';

@Injectable({ providedIn: 'root' })
export class HealthRemoteDataSource {
  getStatus(): HealthStatus {
    return {
      service: 'ngseed-cdp',
      status: 'ok',
      checkedAt: new Date().toISOString(),
    };
  }
}
"#,
    )?;

    write_file(
        &app_dir.join("data/datasource/local/preferences.datasource.ts"),
        r#"import { Injectable } from '@angular/core';

@Injectable({ providedIn: 'root' })
export class PreferencesLocalDataSource {
  private readonly key = 'ngseed:theme';

  getTheme(): string {
    return localStorage.getItem(this.key) ?? 'light';
  }
}
"#,
    )?;

    write_file(
        &app_dir.join("presentation/pages/health/health.page.ts"),
        r#"import { CommonModule } from '@angular/common';
import { Component, inject } from '@angular/core';
import { HealthRemoteDataSource } from '../../../data/datasource/remote/health.datasource';
import { PreferencesLocalDataSource } from '../../../data/datasource/local/preferences.datasource';

@Component({
  selector: 'app-health-page',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './health.page.html',
})
export class HealthPage {
  private readonly remote = inject(HealthRemoteDataSource);
  private readonly local = inject(PreferencesLocalDataSource);

  readonly health = this.remote.getStatus();
  readonly theme = this.local.getTheme();
}
"#,
    )?;

    write_file(
        &app_dir.join("presentation/pages/health/health.page.html"),
        r#"<main class="shell">
  <h1>CDP Architecture Ready</h1>
  <p>Status: {{ health.status }} ({{ health.service }})</p>
  <p>Theme preference: {{ theme }}</p>
</main>
"#,
    )?;

    write_file(
        &app_dir.join("app.routes.ts"),
        r#"import { Routes } from '@angular/router';
import { HealthPage } from './presentation/pages/health/health.page';

export const routes: Routes = [
  {
    path: '',
    component: HealthPage,
  },
];
"#,
    )?;

    patch_app_component_for_cdp(&app_dir)?;
    patch_app_config_for_cdp(&app_dir)?;

    Ok(())
}

fn patch_app_component_for_clean(app_dir: &Path) -> Result<()> {
    let loader = TemplateLoader::new()?;

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

    let context = serde_json::json!({
        "template_url": template_url,
        "style_url": style_url,
        "component_class": component_class
    });

    write_file(&app_ts, &loader.render("app.component.ts.j2", context)?)?;

    write_file(&app_html, &loader.render("app.component.html.j2", ())?)?;

    Ok(())
}

fn patch_app_component_for_cdp(app_dir: &Path) -> Result<()> {
    let (app_ts, app_html, template_url, style_property, component_class) =
        if app_dir.join("app.ts").exists() {
            (
                app_dir.join("app.ts"),
                app_dir.join("app.html"),
                "./app.html",
                "styleUrl",
                "App",
            )
        } else {
            (
                app_dir.join("app.component.ts"),
                app_dir.join("app.component.html"),
                "./app.component.html",
                "styleUrls",
                "AppComponent",
            )
        };

    write_file(&app_ts, &{
        let template = r#"import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: '__TEMPLATE_URL__',
  __STYLE_PROPERTY__: ['./app.scss'],
})
export class __COMPONENT_CLASS__ {}
"#;
        template
            .replace("__TEMPLATE_URL__", template_url)
            .replace("__STYLE_PROPERTY__", style_property)
            .replace("__COMPONENT_CLASS__", component_class)
    })?;

    write_file(
        &app_html,
        r#"<router-outlet />
"#,
    )?;

    Ok(())
}

fn patch_app_config_for_clean(app_dir: &Path) -> Result<()> {
    let loader = TemplateLoader::new()?;
    let app_config = app_dir.join("app.config.ts");

    write_file(&app_config, &loader.render("app.config.ts.j2", ())?)
}

fn patch_app_config_for_cdp(app_dir: &Path) -> Result<()> {
    let app_config = app_dir.join("app.config.ts");

    write_file(
        &app_config,
        r#"import { ApplicationConfig } from '@angular/core';
import { provideRouter } from '@angular/router';

import { routes } from './app.routes';

export const appConfig: ApplicationConfig = {
  providers: [provideRouter(routes)],
};
"#,
    )
}

fn apply_ui_integration(
    runner: &mut dyn CommandRunner,
    project_dir: &Path,
    ui: UiChoice,
    package_manager: PackageManager,
) -> Result<()> {
    match ui {
        UiChoice::None => Ok(()),
        UiChoice::Material => {
            let args = vec![
                "add".to_string(),
                "@angular/material".to_string(),
                "--defaults".to_string(),
                "--skip-confirmation".to_string(),
            ];
            runner.run("ng", &args, Some(project_dir))
        }
        UiChoice::Primeng => {
            let (program, install_args) = package_manager_install_command(
                package_manager,
                &["primeng", "primeicons", "@primeng/themes"],
            );
            runner.run(program, &install_args, Some(project_dir))?;

            add_styles_to_angular_json(
                project_dir,
                &[
                    "node_modules/@primeng/themes/aura/theme.css",
                    "node_modules/primeicons/primeicons.css",
                ],
            )
        }
    }
}

fn apply_styles(
    runner: &mut dyn CommandRunner,
    project_dir: &Path,
    styles: StylesChoice,
    package_manager: PackageManager,
) -> Result<()> {
    match styles {
        StylesChoice::None => Ok(()),
        StylesChoice::TailwindCSS => {
            let (program, install_args) = package_manager_install_command(
                package_manager,
                &["tailwindcss", "postcss", "autoprefixer"],
            );
            runner.run(program, &install_args, Some(project_dir))?;

            // Create tailwind.config.js using the template
            let tailwind_config = project_dir.join("tailwind.config.js");
            let loader = TemplateLoader::new()?;
            fs::write(
                &tailwind_config,
                loader.render("tailwind.config.js.j2", ())?,
            )
            .with_context(|| format!("failed to write {}", tailwind_config.display()))?;

            // Create postcss.config.js
            let postcss_config = project_dir.join("postcss.config.js");
            fs::write(
                &postcss_config,
                r#"export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
"#,
            )
            .with_context(|| format!("failed to write {}", postcss_config.display()))?;

            // Update styles.scss with Tailwind directives
            let styles_scss = project_dir.join("src/styles.scss");
            fs::write(&styles_scss, loader.render("styles.scss.j2", ())?)
                .with_context(|| format!("failed to write {}", styles_scss.display()))?;

            Ok(())
        }
    }
}

fn package_manager_cli_name(package_manager: PackageManager) -> &'static str {
    match package_manager {
        PackageManager::Npm => "npm",
        PackageManager::Pnpm => "pnpm",
        PackageManager::Yarn => "yarn",
        PackageManager::Bun => "bun",
    }
}

fn package_manager_install_command(
    package_manager: PackageManager,
    packages: &[&str],
) -> (&'static str, Vec<String>) {
    match package_manager {
        PackageManager::Npm => {
            let mut args = vec!["install".to_string()];
            args.extend(packages.iter().map(|s| s.to_string()));
            ("npm", args)
        }
        PackageManager::Pnpm => {
            let mut args = vec!["add".to_string()];
            args.extend(packages.iter().map(|s| s.to_string()));
            ("pnpm", args)
        }
        PackageManager::Yarn => {
            let mut args = vec!["add".to_string()];
            args.extend(packages.iter().map(|s| s.to_string()));
            ("yarn", args)
        }
        PackageManager::Bun => {
            let mut args = vec!["add".to_string()];
            args.extend(packages.iter().map(|s| s.to_string()));
            ("bun", args)
        }
    }
}

fn add_styles_to_angular_json(project_dir: &Path, styles: &[&str]) -> Result<()> {
    let angular_json_path = project_dir.join("angular.json");
    let raw = fs::read_to_string(&angular_json_path)
        .with_context(|| format!("failed to read {}", angular_json_path.display()))?;

    let mut json: Value =
        serde_json::from_str(&raw).context("failed to parse angular.json as JSON")?;

    let projects = json
        .get_mut("projects")
        .and_then(Value::as_object_mut)
        .context("angular.json is missing `projects`")?;

    let Some((_project_name, project_config)) = projects.iter_mut().next() else {
        bail!("angular.json has no projects entries");
    };

    let styles_array = project_config
        .pointer_mut("/architect/build/options/styles")
        .and_then(Value::as_array_mut)
        .context("angular.json is missing /architect/build/options/styles")?;

    for style in styles {
        if !styles_array
            .iter()
            .any(|entry| entry.as_str() == Some(style))
        {
            styles_array.push(Value::String((*style).to_string()));
        }
    }

    let rendered = serde_json::to_string_pretty(&json).context("failed to render angular.json")?;
    fs::write(&angular_json_path, format!("{rendered}\n"))
        .with_context(|| format!("failed to write {}", angular_json_path.display()))?;

    Ok(())
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {}", parent.display()))?;
    }

    fs::write(path, content).with_context(|| format!("failed to write {}", path.display()))
}

fn format_command(program: &str, args: &[String]) -> String {
    if args.is_empty() {
        return program.to_string();
    }
    format!("{} {}", program, args.join(" "))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use tempfile::tempdir;

    use super::*;

    #[derive(Default)]
    struct FakeRunner {
        calls: Vec<(String, Vec<String>, Option<PathBuf>)>,
    }

    impl CommandRunner for FakeRunner {
        fn run(&mut self, program: &str, args: &[String], cwd: Option<&Path>) -> Result<()> {
            self.calls.push((
                program.to_string(),
                args.to_vec(),
                cwd.map(Path::to_path_buf),
            ));
            Ok(())
        }
    }

    #[test]
    fn scaffold_calls_ng_new_with_expected_flags() {
        let mut runner = FakeRunner::default();

        scaffold_angular_project(
            &mut runner,
            "demo-app",
            ResolvedOptions {
                ui: UiChoice::None,
                styles: StylesChoice::None,
                package_manager: PackageManager::Pnpm,
                architecture: ArchitectureProfile::Clean,
                skip_install: true,
            },
        )
        .unwrap();

        assert_eq!(runner.calls.len(), 1);
        let (program, args, _) = &runner.calls[0];
        assert_eq!(program, "ng");
        assert!(args.contains(&"new".to_string()));
        assert!(args.contains(&"demo-app".to_string()));
        assert!(args.contains(&"--standalone".to_string()));
        assert!(args.contains(&"--routing".to_string()));
        assert!(args.contains(&"--style=scss".to_string()));
        assert!(args.contains(&"--ssr=false".to_string()));
        assert!(args.contains(&"--package-manager=pnpm".to_string()));
        assert!(args.contains(&"--skip-install".to_string()));
    }

    #[test]
    fn package_manager_install_command_matches_manager() {
        let (program, args) = package_manager_install_command(PackageManager::Bun, &["primeng"]);
        assert_eq!(program, "bun");
        assert_eq!(args, vec!["add".to_string(), "primeng".to_string()]);
    }

    #[test]
    fn primeng_adds_expected_styles() {
        let tmp = tempdir().unwrap();
        let project_dir = tmp.path();
        fs::create_dir_all(project_dir.join("src/app")).unwrap();
        fs::write(
            project_dir.join("angular.json"),
            r#"{
  "projects": {
    "demo": {
      "architect": {
        "build": {
          "options": {
            "styles": ["src/styles.scss"]
          }
        }
      }
    }
  }
}
"#,
        )
        .unwrap();

        add_styles_to_angular_json(
            project_dir,
            &[
                "node_modules/@primeng/themes/aura/theme.css",
                "node_modules/primeicons/primeicons.css",
            ],
        )
        .unwrap();

        let rendered = fs::read_to_string(project_dir.join("angular.json")).unwrap();
        assert!(rendered.contains("node_modules/@primeng/themes/aura/theme.css"));
        assert!(rendered.contains("node_modules/primeicons/primeicons.css"));
    }

    #[test]
    fn clean_template_creates_layered_files() {
        let tmp = tempdir().unwrap();
        let app_dir = tmp.path().join("demo/src/app");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("app.ts"), "").unwrap();
        fs::write(app_dir.join("app.html"), "").unwrap();
        fs::write(app_dir.join("app.config.ts"), "").unwrap();

        apply_clean_architecture_template(&tmp.path().join("demo")).unwrap();

        assert!(
            app_dir
                .join("domain/ports/greeting-repository.port.ts")
                .exists()
        );
        assert!(
            app_dir
                .join("application/use-cases/get-greeting.use-case.ts")
                .exists()
        );
        assert!(
            app_dir
                .join("infrastructure/providers/greeting.provider.ts")
                .exists()
        );
        assert!(app_dir.join("presentation/facades/home.facade.ts").exists());
    }

    #[test]
    fn cdp_template_creates_layered_files() {
        let tmp = tempdir().unwrap();
        let app_dir = tmp.path().join("demo/src/app");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("app.ts"), "").unwrap();
        fs::write(app_dir.join("app.html"), "").unwrap();
        fs::write(app_dir.join("app.config.ts"), "").unwrap();
        fs::write(app_dir.join("app.routes.ts"), "").unwrap();

        apply_cdp_architecture_template(&tmp.path().join("demo")).unwrap();

        assert!(app_dir.join("core/models/health-status.model.ts").exists());
        assert!(
            app_dir
                .join("data/datasource/remote/health.datasource.ts")
                .exists()
        );
        assert!(
            app_dir
                .join("presentation/pages/health/health.page.ts")
                .exists()
        );
    }
}
