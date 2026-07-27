use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

use crate::domain::project::ArchitectureProfile;
use crate::domain::project::NewProjectRequest;
use crate::domain::project::PackageManager;
use crate::domain::project::UiChoice;
use crate::domain::styles_choice::StylesChoice;

#[derive(Parser, Debug)]
#[command(
    name = "ngseed",
    version,
    about = "Initialize Angular projects with a clean architecture baseline",
    long_about = "A modern CLI to scaffold Angular projects, apply architecture templates, and integrate a UI stack.",
    after_help = "Examples:\n  ngseed new my-app --architecture clean\n  ngseed new my-app --architecture cdp --ui none\n  ngseed new my-app --yes --ui material --package-manager pnpm",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New(NewCommand),
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
    Cdp,
}

pub enum AppCommand {
    New(NewProjectRequest),
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
            CliArchitectureProfile::Cdp => ArchitectureProfile::Cdp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_new_command_with_all_flags() {
        let command = parse_from([
            "ngseed",
            "new",
            "demo",
            "--yes",
            "--skip-install",
            "--ui",
            "primeng",
            "--package-manager",
            "pnpm",
            "--architecture",
            "cdp",
        ])
        .unwrap();

        let AppCommand::New(request) = command;
        assert_eq!(request.project_name, "demo");
        assert_eq!(request.ui, Some(UiChoice::Primeng));
        assert_eq!(request.package_manager, Some(PackageManager::Pnpm));
        assert_eq!(request.architecture, Some(ArchitectureProfile::Cdp));
        assert!(request.skip_install);
        assert!(request.yes);
    }
}
