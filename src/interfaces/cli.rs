use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

use crate::domain::project::ArchitectureProfile;
use crate::domain::project::DocsEngine;
use crate::domain::project::Framework;
use crate::domain::project::GenerateFeatureRequest;
use crate::domain::project::NewProjectRequest;
use crate::domain::project::PackageManager;
use crate::domain::project::UiChoice;
use crate::domain::styles_choice::StylesChoice;

#[derive(Parser, Debug)]
#[command(
    name = "brota",
    version,
    about = "Scaffold production-ready Angular projects",
    long_about = "A modern CLI to scaffold Angular projects, apply architecture templates, and integrate a UI stack.",
    after_help = "Examples:\n  brota new my-app --architecture clean\n  brota new my-app --architecture ddd --ui none\n  brota new my-app --yes --ui material --package-manager pnpm",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New(NewCommand),
    Generate(GenerateCommand),
}

#[derive(Parser, Debug)]
struct GenerateCommand {
    #[command(subcommand)]
    sub: GenerateSubCommand,
}

#[derive(Subcommand, Debug)]
enum GenerateSubCommand {
    Feature(GenerateFeatureCommand),
}

#[derive(Parser, Debug)]
struct GenerateFeatureCommand {
    name: String,

    #[arg(long, value_enum)]
    ui: Option<CliUiChoice>,

    #[arg(long, value_enum)]
    styles: Option<CliStylesChoice>,

    #[arg(long, value_enum)]
    architecture: Option<CliArchitectureProfile>,

    #[arg(long, default_value = "api")]
    prefix: String,

    #[arg(long, value_delimiter = ',')]
    fields: Vec<String>,

    #[arg(long)]
    project_dir: Option<String>,
}

#[derive(Parser, Debug)]
struct NewCommand {
    project_name: String,

    #[arg(long, value_enum)]
    ui: Option<CliUiChoice>,

    #[arg(long, value_enum)]
    styles: Option<CliStylesChoice>,

    #[arg(long, value_enum)]
    package_manager: Option<CliPackageManager>,

    #[arg(long, value_enum)]
    architecture: Option<CliArchitectureProfile>,

    #[arg(long)]
    skip_install: bool,

    #[arg(long)]
    skip_git: bool,

    #[arg(long)]
    yes: bool,

    /// Target frontend framework (angular | astro). Defaults to angular.
    #[arg(long, value_enum)]
    framework: Option<CliFramework>,

    /// Docs engine for Astro projects (starlight | native). Defaults to starlight.
    /// Ignored when --framework is angular.
    #[arg(long, value_enum)]
    docs_engine: Option<CliDocsEngine>,

    /// Comma-separated list of locales for Astro docs i18n, e.g. `--i18n en,es`.
    /// The first entry is the default locale. Ignored when --framework is angular.
    #[arg(long, value_delimiter = ',')]
    i18n: Vec<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CliUiChoice {
    Material,
    Primeng,
    None,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CliStylesChoice {
    Css,
    Scss,
    Sass,
    Less,
    Tailwindcss,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CliPackageManager {
    Npm,
    Pnpm,
    Yarn,
    Bun,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CliArchitectureProfile {
    Clean,
    Ddd,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CliFramework {
    Angular,
    Astro,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CliDocsEngine {
    Starlight,
    Native,
}

pub enum AppCommand {
    New(NewProjectRequest),
    GenerateFeature(GenerateFeatureRequest),
}

pub fn parse() -> Result<AppCommand> {
    Ok(map_cli_to_command(Cli::parse()))
}

#[cfg(test)]
pub fn parse_from<I, T>(itr: I) -> Result<AppCommand>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    Ok(map_cli_to_command(Cli::parse_from(itr)))
}

fn map_cli_to_command(cli: Cli) -> AppCommand {
    match cli.command {
        Commands::New(cmd) => AppCommand::New(NewProjectRequest {
            project_name: cmd.project_name,
            ui: cmd.ui.map(Into::into),
            styles: cmd.styles.map(Into::into),
            package_manager: cmd.package_manager.map(Into::into),
            architecture: cmd.architecture.map(Into::into),
            skip_install: cmd.skip_install,
            skip_git: cmd.skip_git,
            yes: cmd.yes,
            framework: cmd.framework.map(Into::into),
            docs_engine: cmd.docs_engine.map(Into::into),
            locales: cmd.i18n,
        }),
        Commands::Generate(cmd) => match cmd.sub {
            GenerateSubCommand::Feature(sub) => {
                let project_dir = sub.project_dir.unwrap_or_else(|| {
                    std::env::current_dir()
                        .expect("failed to get current dir")
                        .to_string_lossy()
                        .to_string()
                });
                AppCommand::GenerateFeature(GenerateFeatureRequest {
                    project_dir,
                    name: sub.name,
                    ui: sub.ui.map(Into::into),
                    styles: sub.styles.map(Into::into),
                    architecture: sub
                        .architecture
                        .map_or(ArchitectureProfile::Clean, Into::into),
                    prefix: sub.prefix,
                    fields: sub.fields,
                })
            }
        },
    }
}

impl From<CliUiChoice> for UiChoice {
    fn from(value: CliUiChoice) -> Self {
        match value {
            CliUiChoice::Material => UiChoice::Material,
            CliUiChoice::Primeng => UiChoice::Primeng,
            CliUiChoice::None => UiChoice::None,
        }
    }
}

impl From<CliStylesChoice> for StylesChoice {
    fn from(value: CliStylesChoice) -> Self {
        match value {
            CliStylesChoice::Css => StylesChoice::Css,
            CliStylesChoice::Scss => StylesChoice::Scss,
            CliStylesChoice::Sass => StylesChoice::Sass,
            CliStylesChoice::Less => StylesChoice::Less,
            CliStylesChoice::Tailwindcss => StylesChoice::TailwindCSS,
        }
    }
}

impl From<CliPackageManager> for PackageManager {
    fn from(value: CliPackageManager) -> Self {
        match value {
            CliPackageManager::Npm => PackageManager::Npm,
            CliPackageManager::Pnpm => PackageManager::Pnpm,
            CliPackageManager::Yarn => PackageManager::Yarn,
            CliPackageManager::Bun => PackageManager::Bun,
        }
    }
}

impl From<CliArchitectureProfile> for ArchitectureProfile {
    fn from(value: CliArchitectureProfile) -> Self {
        match value {
            CliArchitectureProfile::Clean => ArchitectureProfile::Clean,
            CliArchitectureProfile::Ddd => ArchitectureProfile::Ddd,
        }
    }
}

impl From<CliFramework> for Framework {
    fn from(value: CliFramework) -> Self {
        match value {
            CliFramework::Angular => Framework::Angular,
            CliFramework::Astro => Framework::Astro,
        }
    }
}

impl From<CliDocsEngine> for DocsEngine {
    fn from(value: CliDocsEngine) -> Self {
        match value {
            CliDocsEngine::Starlight => DocsEngine::Starlight,
            CliDocsEngine::Native => DocsEngine::Native,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_new_command_with_all_flags() {
        let command = parse_from([
            "brota",
            "new",
            "demo",
            "--yes",
            "--skip-install",
            "--ui",
            "primeng",
            "--package-manager",
            "pnpm",
            "--architecture",
            "ddd",
        ])
        .unwrap();

        let request = match command {
            AppCommand::New(r) => r,
            _ => panic!("expected New command"),
        };
        assert_eq!(request.project_name, "demo");
        assert_eq!(request.ui, Some(UiChoice::Primeng));
        assert_eq!(request.package_manager, Some(PackageManager::Pnpm));
        assert_eq!(request.architecture, Some(ArchitectureProfile::Ddd));
        assert!(request.skip_install);
        assert!(request.yes);
    }

    #[test]
    fn parse_new_command_with_tailwind_styles() {
        let command = parse_from(["brota", "new", "demo", "--styles", "tailwindcss"]).unwrap();

        let request = match command {
            AppCommand::New(r) => r,
            _ => panic!("expected New command"),
        };

        assert_eq!(request.styles, Some(StylesChoice::TailwindCSS));
    }

    #[test]
    fn parse_new_command_defaults_framework_to_none() {
        let command = parse_from(["brota", "new", "demo", "--yes"]).unwrap();

        let request = match command {
            AppCommand::New(r) => r,
            _ => panic!("expected New command"),
        };

        assert_eq!(request.framework, None);
        assert_eq!(request.docs_engine, None);
        assert!(request.locales.is_empty());
    }

    #[test]
    fn parse_new_command_with_astro_framework_defaults_engine_to_none() {
        let command = parse_from(["brota", "new", "docs", "--framework", "astro"]).unwrap();

        let request = match command {
            AppCommand::New(r) => r,
            _ => panic!("expected New command"),
        };

        assert_eq!(request.framework, Some(Framework::Astro));
        assert_eq!(request.docs_engine, None);
        assert!(request.locales.is_empty());
    }

    #[test]
    fn parse_new_command_with_astro_starlight_and_i18n_locales() {
        let command = parse_from([
            "brota",
            "new",
            "docs",
            "--framework",
            "astro",
            "--docs-engine",
            "starlight",
            "--i18n",
            "en,es",
        ])
        .unwrap();

        let request = match command {
            AppCommand::New(r) => r,
            _ => panic!("expected New command"),
        };

        assert_eq!(request.framework, Some(Framework::Astro));
        assert_eq!(request.docs_engine, Some(DocsEngine::Starlight));
        assert_eq!(request.locales, vec!["en", "es"]);
    }

    #[test]
    fn parse_new_command_with_astro_native_engine_without_locales() {
        let command = parse_from([
            "brota",
            "new",
            "docs",
            "--framework",
            "astro",
            "--docs-engine",
            "native",
        ])
        .unwrap();

        let request = match command {
            AppCommand::New(r) => r,
            _ => panic!("expected New command"),
        };

        assert_eq!(request.framework, Some(Framework::Astro));
        assert_eq!(request.docs_engine, Some(DocsEngine::Native));
        assert!(request.locales.is_empty());
    }

    #[test]
    fn parse_generate_feature_minimal() {
        let command = parse_from(["brota", "generate", "feature", "user"]).unwrap();

        let request = match command {
            AppCommand::GenerateFeature(r) => r,
            _ => panic!("expected GenerateFeature command"),
        };
        assert_eq!(request.name, "user");
        assert_eq!(request.architecture, ArchitectureProfile::Clean);
        assert_eq!(request.prefix, "api");
        assert!(request.fields.is_empty());
    }

    #[test]
    fn parse_generate_feature_with_fields() {
        let command = parse_from([
            "brota",
            "generate",
            "feature",
            "product",
            "--fields",
            "name:string,price:number",
            "--prefix",
            "v1",
            "--architecture",
            "ddd",
        ])
        .unwrap();

        let request = match command {
            AppCommand::GenerateFeature(r) => r,
            _ => panic!("expected GenerateFeature command"),
        };
        assert_eq!(request.name, "product");
        assert_eq!(request.architecture, ArchitectureProfile::Ddd);
        assert_eq!(request.prefix, "v1");
        assert_eq!(request.fields, vec!["name:string", "price:number"]);
    }

    #[test]
    fn parse_generate_feature_without_fields_and_architecture() {
        let command = parse_from(["brota", "generate", "feature", "my-feature"]).unwrap();

        let request = match command {
            AppCommand::GenerateFeature(r) => r,
            _ => panic!("expected GenerateFeature command"),
        };
        assert_eq!(request.name, "my-feature");
        assert_eq!(request.architecture, ArchitectureProfile::Clean);
        assert_eq!(request.prefix, "api");
        assert!(request.fields.is_empty());
    }
}
