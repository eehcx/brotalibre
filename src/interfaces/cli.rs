use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

use crate::domain::project::ArchitectureProfile;
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
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CliUiChoice {
    Material,
    Primeng,
    None,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CliStylesChoice {
    Tailwindcss,
    None,
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
            CliStylesChoice::Tailwindcss => StylesChoice::TailwindCSS,
            CliStylesChoice::None => StylesChoice::None,
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
        let command = parse_from([
            "brota",
            "generate",
            "feature",
            "my-feature",
        ])
        .unwrap();

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
