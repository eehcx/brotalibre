use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use crate::application::ports::ProgressReporter;
use crate::domain::project::{ArchitectureProfile, PackageManager, ResolvedOptions, UiChoice};
use crate::domain::styles_choice::StylesChoice;

pub struct ConsoleProgressReporter {
    spinner: Mutex<Option<ProgressBar>>,
}

impl Default for ConsoleProgressReporter {
    fn default() -> Self {
        Self {
            spinner: Mutex::new(None),
        }
    }
}

impl ProgressReporter for ConsoleProgressReporter {
    /*fn show_banner(&self) {
        println!(
            "{}",
            style(" _   _  ____ ____  _____ _____ ____  ").cyan().bold()
        );
        println!(
            "{}",
            style("| \\ | |/ ___/ ___|| ____| ____|  _ \\ ")
                .cyan()
                .bold()
        );
        println!(
            "{}",
            style("|  \\| | |  _\\___ \\|  _| |  _| | | | |")
                .cyan()
                .bold()
        );
        println!(
            "{}",
            style("| |\\  | |_| |___) | |___| |___| |_| |")
                .cyan()
                .bold()
        );
        println!(
            "{}",
            style("|_| \\_|\\____|____/|_____|_____|____/ ")
                .cyan()
                .bold()
        );
        println!("{}", style("Angular project bootstrap CLI").dim());
        println!();
    }*/

    fn stage_start(&self, stage: &str, message: &str) {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner} {msg}")
                .unwrap_or_else(|_| ProgressStyle::default_spinner()),
        );
        spinner.enable_steady_tick(Duration::from_millis(90));
        spinner.set_message(format!(
            "{} {} {}",
            style("[stage]").cyan().bold(),
            style(stage).bold(),
            style(message).dim()
        ));

        if let Ok(mut current) = self.spinner.lock() {
            *current = Some(spinner);
        }
    }

    fn stage_ok(&self, stage: &str, message: &str) {
        if let Ok(mut current) = self.spinner.lock()
            && let Some(spinner) = current.take()
        {
            spinner.finish_and_clear();
        }

        println!(
            "{} {} {}",
            style("✔").green().bold(),
            style(stage).bold(),
            style(message).dim()
        );
    }

    fn stage_error(&self, stage: &str, message: &str) {
        if let Ok(mut current) = self.spinner.lock()
            && let Some(spinner) = current.take()
        {
            spinner.finish_and_clear();
        }

        eprintln!(
            "{} {} {}",
            style("✖").red().bold(),
            style(stage).bold(),
            style(message).dim()
        );
    }

    fn summary(&self, project_name: &str, project_dir: &Path, options: ResolvedOptions) {
        println!();
        println!("{}", style("done").green().bold());
        println!(
            "{} {}",
            style("project:").bold(),
            style(project_name).cyan().bold()
        );
        println!("{} {}", style("path:").bold(), project_dir.display());
        println!(
            "{} {}",
            style("ui:").bold(),
            style(ui_label(options.ui)).yellow()
        );
        println!(
            "{} {}",
            style("package manager:").bold(),
            style(package_manager_label(options.package_manager)).yellow()
        );
        println!(
            "{} {}",
            style("architecture:").bold(),
            style(architecture_label(options.architecture)).yellow()
        );
        println!(
            "{} {}",
            style("skip install:").bold(),
            style(if options.skip_install { "yes" } else { "no" }).yellow()
        );
        println!(
            "{} {}",
            style("styles:").bold(),
            style(styles_label(options.styles)).yellow()
        );
        println!();
        println!("{}", style("next steps:").bold());
        println!("  cd {}", project_name);
        println!("  ng serve (recommended)");
        //println!("  {} start or ng serve (recommended)", package_manager_label(options.package_manager));
        //println!("  ng serve (recommended)");
    }
}

fn ui_label(ui: UiChoice) -> &'static str {
    match ui {
        UiChoice::Material => "material",
        UiChoice::Primeng => "primeng",
        UiChoice::None => "none",
    }
}

fn package_manager_label(pm: PackageManager) -> &'static str {
    match pm {
        PackageManager::Npm => "npm",
        PackageManager::Pnpm => "pnpm",
        PackageManager::Yarn => "yarn",
        PackageManager::Bun => "bun",
    }
}

fn architecture_label(profile: ArchitectureProfile) -> &'static str {
    match profile {
        ArchitectureProfile::Clean => "clean",
        ArchitectureProfile::Ddd => "ddd",
    }
}

fn styles_label(styles: StylesChoice) -> &'static str {
    match styles {
        StylesChoice::Css => "css",
        StylesChoice::Scss => "scss",
        StylesChoice::Sass => "sass",
        StylesChoice::Less => "less",
        StylesChoice::TailwindCSS => "tailwindcss",
    }
}
