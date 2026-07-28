use anyhow::{Context, Result};
use dialoguer::{Select, theme::ColorfulTheme};

use crate::application::ports::UiSelector;
use crate::domain::project::ArchitectureProfile;
use crate::domain::project::PackageManager;
use crate::domain::project::UiChoice;
use crate::domain::styles_choice::StylesChoice;

pub struct DialoguerUiSelector;

impl UiSelector for DialoguerUiSelector {
    fn select_ui(&self) -> Result<UiChoice> {
        let choices = ["None", "Angular Material", "PrimeNG"];
        let selected = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select UI library")
            .items(&choices)
            .default(0)
            .interact()
            .context("failed to read UI selection")?;

        let ui = match selected {
            0 => UiChoice::None,
            1 => UiChoice::Material,
            2 => UiChoice::Primeng,
            _ => UiChoice::None,
        };

        Ok(ui)
    }

    fn select_styles(&self) -> Result<StylesChoice> {
        let choices = ["None", "TailwindCSS"];
        let selected = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select styles option")
            .items(&choices)
            .default(0)
            .interact()
            .context("failed to read styles selection")?;

        let styles = match selected {
            0 => StylesChoice::None,
            1 => StylesChoice::TailwindCSS,
            _ => StylesChoice::None,
        };

        Ok(styles)
    }

    fn select_package_manager(&self) -> Result<PackageManager> {
        let choices = ["npm", "pnpm", "yarn", "bun"];
        let selected = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select package manager")
            .items(&choices)
            .default(0)
            .interact()
            .context("failed to read package manager selection")?;

        let manager = match selected {
            0 => PackageManager::Npm,
            1 => PackageManager::Pnpm,
            2 => PackageManager::Yarn,
            3 => PackageManager::Bun,
            _ => PackageManager::Npm,
        };

        Ok(manager)
    }

    fn select_architecture(&self) -> Result<ArchitectureProfile> {
        let choices = ["clean", "ddd"];
        let selected = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select architecture profile")
            .items(&choices)
            .default(0)
            .interact()
            .context("failed to read architecture selection")?;

        let profile = match selected {
            0 => ArchitectureProfile::Clean,
            1 => ArchitectureProfile::Ddd,
            _ => ArchitectureProfile::Clean,
        };

        Ok(profile)
    }
}
