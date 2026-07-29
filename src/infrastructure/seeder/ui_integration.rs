use std::path::Path;

use anyhow::Result;

use crate::domain::project::PackageManager;
use crate::domain::project::UiChoice;
use crate::infrastructure::seeder::CommandRunner;
use crate::infrastructure::seeder::commands::add_styles_to_angular_json;
use crate::infrastructure::seeder::commands::package_manager_install_command;

pub(crate) fn apply_ui_integration(
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
