use anyhow::{Context, Result};
use dialoguer::{Input, Select, theme::ColorfulTheme};

use crate::application::ports::UiSelector;
use crate::domain::project::ArchitectureProfile;
use crate::domain::project::DocsEngine;
use crate::domain::project::Framework;
use crate::domain::project::PackageManager;
use crate::domain::project::UiChoice;
use crate::domain::styles_choice::StylesChoice;

pub struct DialoguerUiSelector;

impl UiSelector for DialoguerUiSelector {
    fn select_framework(&self) -> Result<Framework> {
        let choices = [
            "Angular (web applications and CRUDs)",
            "Astro (documentation sites)",
        ];
        let selected = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What do you want to create?")
            .items(&choices)
            .default(0)
            .interact()
            .context("failed to read framework selection")?;

        Ok(match selected {
            1 => Framework::Astro,
            _ => Framework::Angular,
        })
    }

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
        let choices = ["CSS (default)", "SCSS", "Sass", "Less", "TailwindCSS"];
        let selected = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select styles option")
            .items(&choices)
            .default(0)
            .interact()
            .context("failed to read styles selection")?;

        let styles = match selected {
            0 => StylesChoice::Css,
            1 => StylesChoice::Scss,
            2 => StylesChoice::Sass,
            3 => StylesChoice::Less,
            4 => StylesChoice::TailwindCSS,
            _ => StylesChoice::Css,
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

    fn select_docs_engine(&self) -> Result<DocsEngine> {
        let choices = [
            "Starlight (recommended, full docs theme)",
            "Native Astro (maximum control)",
        ];
        let selected = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select documentation engine")
            .items(&choices)
            .default(0)
            .interact()
            .context("failed to read documentation engine selection")?;

        Ok(match selected {
            1 => DocsEngine::Native,
            _ => DocsEngine::Starlight,
        })
    }

    fn select_locales(&self) -> Result<Vec<String>> {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Documentation locales (comma-separated)")
            .default("en".to_string())
            .interact_text()
            .context("failed to read documentation locales")?;

        let locales = input
            .split(',')
            .map(str::trim)
            .filter(|locale| !locale.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>();

        if locales.is_empty() {
            anyhow::bail!("at least one documentation locale is required");
        }

        Ok(locales)
    }
}
