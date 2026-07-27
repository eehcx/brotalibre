use anyhow::Result;

mod application;
mod domain;
mod infrastructure;
mod interfaces;

use application::use_cases::new_project::NewProjectUseCase;
use infrastructure::console_progress_reporter::ConsoleProgressReporter;
use infrastructure::dialoguer_ui_selector::DialoguerUiSelector;
use infrastructure::system_environment::SystemEnvironment;
use infrastructure::seeder::SystemSeeder;

const BANNER: &str = r#"
                                   _ 
                                  | |
     _ __   __ _ ___  ___  ___  __| |
    | '_ \ / _` / __|/ _ \/ _ \/ _` |
    | | | | (_| \__ \  __/  __/ (_| |
    |_| |_|\__, |___/\___|\___|\__,_|
            __/ |                    
           |___/                     
"#;

pub fn run() -> Result<()> {
    println!("{}", BANNER);

    let command = interfaces::cli::parse()?;

    match command {
        interfaces::cli::AppCommand::New(request) => {
            let env = SystemEnvironment;
            let ui_selector = DialoguerUiSelector;
            let seeder = SystemSeeder;
            let reporter = ConsoleProgressReporter::default();

            let use_case = NewProjectUseCase::new(&env, &ui_selector, &seeder, &reporter);
            use_case.execute(request)
        }
    }
}
