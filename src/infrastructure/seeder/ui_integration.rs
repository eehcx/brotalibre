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
            runner.run("ng", &args, Some(project_dir))?;

            let animations_package = angular_animations_package(project_dir)?;
            let (program, install_args) =
                package_manager_install_command(package_manager, &[animations_package.as_str()]);
            runner.run(program, &install_args, Some(project_dir))
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

fn angular_animations_package(project_dir: &Path) -> Result<String> {
    let package_json = project_dir.join("package.json");
    let content = std::fs::read_to_string(&package_json)?;
    let package: serde_json::Value = serde_json::from_str(&content)?;
    let angular_core = package
        .get("dependencies")
        .and_then(|dependencies| dependencies.get("@angular/core"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("22");
    let major = angular_core
        .trim_start_matches(['^', '~', '=', '>', '<'])
        .split('.')
        .next()
        .filter(|value| value.chars().all(|character| character.is_ascii_digit()))
        .unwrap_or("22");

    Ok(format!("@angular/animations@{major}"))
}
