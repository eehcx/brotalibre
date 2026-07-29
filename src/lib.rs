use anyhow::Result;
use console::style;

mod application;
mod domain;
mod infrastructure;
mod interfaces;

use application::use_cases::generate_feature::GenerateFeatureUseCase;
use application::use_cases::new_project::NewProjectUseCase;
use infrastructure::console_progress_reporter::ConsoleProgressReporter;
use infrastructure::dialoguer_ui_selector::DialoguerUiSelector;
use infrastructure::seeder::SystemSeeder;
use infrastructure::system_environment::SystemEnvironment;

const BANNER: &str = r#"

    ______           _        _     _ _
    | ___ \         | |      | |   (_) |
    | |_/ /_ __ ___ | |_ __ _| |    _| |__  _ __ ___
    | ___ \ '__/ _ \| __/ _` | |   | | '_ \| '__/ _ \
    | |_/ / | | (_) | || (_| | |___| | |_) | | |  __/
    \____/|_|  \___/ \__\__,_\_____/_|_.__/|_|  \___|

    BrotaLibre - Grow scalable frontends from solid foundations.
"#;

pub fn run() -> Result<()> {
    println!("{}", style(BANNER).blue());

    let command = interfaces::cli::parse()?;

    match command {
        interfaces::cli::AppCommand::New(request) => {
            let env = SystemEnvironment;
            let ui_selector = DialoguerUiSelector;
            let seeder = SystemSeeder;
            let astro_seeder = SystemSeeder;
            let reporter = ConsoleProgressReporter::default();

            let use_case =
                NewProjectUseCase::new(&env, &ui_selector, &seeder, &astro_seeder, &reporter);
            use_case.execute(request)
        }
        interfaces::cli::AppCommand::GenerateFeature(request) => {
            let seeder = SystemSeeder;
            let reporter = ConsoleProgressReporter::default();

            let use_case = GenerateFeatureUseCase::new(&seeder, &reporter);
            use_case.execute(request)
        }
    }
}
