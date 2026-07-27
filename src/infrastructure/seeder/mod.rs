use std::path::Path;

use anyhow::Result;

pub(crate) mod commands;
pub(crate) mod styles;
pub(crate) mod templates;
pub(crate) mod ui_integration;

use crate::application::ports::Seeder;
use crate::domain::project::ArchitectureProfile;
use crate::domain::project::PackageManager;
use crate::domain::project::ResolvedOptions;
use crate::domain::project::UiChoice;
use crate::domain::styles_choice::StylesChoice;
use crate::infrastructure::seeder::styles::apply_styles;
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
    ) -> Result<()> {
        let template_base = std::env::current_dir()
            .map(|p| p.join("templates").join("angular"))
            .expect("Failed to get current directory");

        match architecture {
            ArchitectureProfile::Clean => {
                templates::clean::apply_clean_architecture_template(
                    &template_base,
                    project_dir,
                    project_name,
                )
            }
            ArchitectureProfile::Cdp => {
                templates::cdp::apply_cdp_architecture_template(project_dir)
            }
        }
    }

    fn apply_ui_integration(
        &self,
        project_dir: &Path,
        ui: UiChoice,
        package_manager: PackageManager,
    ) -> Result<()> {
        let template_base = std::env::current_dir()
            .map(|p| p.join("templates").join("angular"))
            .expect("Failed to get current directory");

        let mut runner = SystemCommandRunner;
        apply_ui_integration(
            &mut runner,
            &template_base,
            project_dir,
            ui,
            package_manager,
        )
    }

    fn apply_styles(
        &self,
        project_dir: &Path,
        styles: StylesChoice,
        package_manager: PackageManager,
    ) -> Result<()> {
        let template_base = std::env::current_dir()
            .map(|p| p.join("templates").join("angular"))
            .expect("Failed to get current directory");

        let mut runner = SystemCommandRunner;
        apply_styles(
            &mut runner,
            &template_base,
            project_dir,
            styles,
            package_manager,
        )
    }
}
