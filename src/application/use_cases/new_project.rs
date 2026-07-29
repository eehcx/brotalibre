use anyhow::{Result, bail};

use crate::application::ports::Environment;
use crate::application::ports::ProgressReporter;
use crate::application::ports::Seeder;
use crate::application::ports::UiSelector;
use crate::domain::project::ArchitectureProfile;
use crate::domain::project::NewProjectRequest;
use crate::domain::project::PackageManager;
use crate::domain::project::ResolvedOptions;
use crate::domain::project::UiChoice;
use crate::domain::styles_choice::StylesChoice;

pub struct NewProjectUseCase<'a> {
    env: &'a dyn Environment,
    ui_selector: &'a dyn UiSelector,
    seeder: &'a dyn Seeder,
    reporter: &'a dyn ProgressReporter,
}

impl<'a> NewProjectUseCase<'a> {
    pub fn new(
        env: &'a dyn Environment,
        ui_selector: &'a dyn UiSelector,
        seeder: &'a dyn Seeder,
        reporter: &'a dyn ProgressReporter,
    ) -> Self {
        Self {
            env,
            ui_selector,
            seeder,
            reporter,
        }
    }

    pub fn execute(&self, request: NewProjectRequest) -> Result<()> {
        let options = self.resolve_options(&request)?;

        if !request.yes && !self.env.is_ci() && self.env.is_interactive_terminal() {
            //self.reporter.show_banner();
        }

        self.reporter
            .stage_start("preflight", "checking required tools");
        if let Err(err) = self.seeder.ensure_required_tools(options.package_manager) {
            self.reporter
                .stage_error("preflight", "required tool check failed");
            return Err(err);
        }
        self.reporter
            .stage_ok("preflight", "required tools look good");

        if self.env.project_exists(&request.project_name) {
            bail!(
                "project directory `{}` already exists. Choose a different project name.",
                request.project_name
            );
        }

        self.reporter
            .stage_start("scaffold", "creating Angular project");
        if let Err(err) = self
            .seeder
            .scaffold_angular_project(&request.project_name, options)
        {
            self.reporter
                .stage_error("scaffold", "Angular scaffolding failed");
            return Err(err);
        }
        self.reporter
            .stage_ok("scaffold", "Angular project created");

        let absolute_project_dir = self.env.current_dir()?.join(&request.project_name);

        self.reporter
            .stage_start("template", "applying architecture template");
        if let Err(err) = self.seeder.apply_architecture_template(
            &absolute_project_dir,
            options.architecture,
            &request.project_name,
            options.styles,
        ) {
            self.reporter
                .stage_error("template", "template setup failed");
            return Err(err);
        }
        self.reporter
            .stage_ok("template", "architecture template applied");

        self.reporter
            .stage_start("feature", "creating the default users feature");
        let default_user_fields = [
            "name:string",
            "email:string",
            "role:string",
            "createdAt:string",
        ];
        if let Err(err) = self.seeder.apply_feature_template(
            &absolute_project_dir,
            options.architecture,
            "users",
            "api/users",
            &default_user_fields.map(String::from),
            options.ui,
            options.styles,
        ) {
            self.reporter
                .stage_error("feature", "default users feature failed");
            return Err(err);
        }
        self.reporter
            .stage_ok("feature", "default users feature created");

        self.reporter
            .stage_start("ui setup", "applying selected UI integration");
        if let Err(err) = self.seeder.apply_ui_integration(
            &absolute_project_dir,
            options.ui,
            options.package_manager,
        ) {
            self.reporter
                .stage_error("ui setup", "UI integration failed");
            return Err(err);
        }
        self.reporter
            .stage_ok("ui setup", "UI integration completed");

        self.reporter
            .summary(&request.project_name, &absolute_project_dir, options);

        Ok(())
    }

    fn resolve_options(&self, request: &NewProjectRequest) -> Result<ResolvedOptions> {
        let package_manager = if let Some(value) = request.package_manager {
            value
        } else if request.yes {
            PackageManager::Npm
        } else {
            self.ui_selector.select_package_manager()?
        };

        let architecture = if let Some(value) = request.architecture {
            value
        } else if request.yes {
            ArchitectureProfile::Clean
        } else {
            self.ui_selector.select_architecture()?
        };

        if request.yes {
            return Ok(ResolvedOptions {
                ui: request.ui.unwrap_or(UiChoice::None),
                styles: request.styles.unwrap_or(StylesChoice::Css),
                package_manager,
                architecture,
                skip_install: request.skip_install,
                skip_git: request.skip_git,
            });
        }

        let ui = if let Some(value) = request.ui {
            value
        } else {
            self.ui_selector.select_ui()?
        };

        let styles = if let Some(value) = request.styles {
            value
        } else {
            self.ui_selector.select_styles()?
        };

        Ok(ResolvedOptions {
            ui,
            styles,
            package_manager,
            architecture,
            skip_install: request.skip_install,
            skip_git: request.skip_git,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::path::Path;
    use std::path::PathBuf;

    use super::*;
    use crate::application::ports::Environment;
    use crate::application::ports::ProgressReporter;
    use crate::application::ports::Seeder;
    use crate::application::ports::UiSelector;
    use crate::domain::project::NewProjectRequest;
    use crate::domain::project::PackageManager;

    struct FakeUiSelector {
        ui: UiChoice,
        styles: StylesChoice,
    }

    impl UiSelector for FakeUiSelector {
        fn select_ui(&self) -> Result<UiChoice> {
            Ok(self.ui)
        }

        fn select_styles(&self) -> Result<StylesChoice> {
            Ok(self.styles)
        }

        fn select_package_manager(&self) -> Result<PackageManager> {
            Ok(PackageManager::Npm)
        }

        fn select_architecture(&self) -> Result<ArchitectureProfile> {
            Ok(ArchitectureProfile::Clean)
        }
    }

    struct FakeEnvironment {
        exists: bool,
        cwd: PathBuf,
    }

    impl Environment for FakeEnvironment {
        fn project_exists(&self, _project_name: &str) -> bool {
            self.exists
        }

        fn current_dir(&self) -> Result<PathBuf> {
            Ok(self.cwd.clone())
        }

        fn is_ci(&self) -> bool {
            false
        }

        fn is_interactive_terminal(&self) -> bool {
            true
        }
    }

    #[derive(Default)]
    struct FakeSeeder {
        calls: RefCell<Vec<String>>,
    }

    impl Seeder for FakeSeeder {
        fn ensure_required_tools(&self, _package_manager: PackageManager) -> Result<()> {
            self.calls
                .borrow_mut()
                .push("ensure_required_tools".to_string());
            Ok(())
        }

        fn scaffold_angular_project(
            &self,
            _project_name: &str,
            _options: ResolvedOptions,
        ) -> Result<()> {
            self.calls
                .borrow_mut()
                .push("scaffold_angular_project".to_string());
            Ok(())
        }

        fn apply_architecture_template(
            &self,
            _project_dir: &Path,
            _architecture: ArchitectureProfile,
            _project_name: &str,
            _styles: StylesChoice,
        ) -> Result<()> {
            self.calls
                .borrow_mut()
                .push("apply_architecture_template".to_string());
            Ok(())
        }

        fn apply_feature_template(
            &self,
            _project_dir: &Path,
            _architecture: ArchitectureProfile,
            _name: &str,
            _prefix: &str,
            _fields: &[String],
            _ui: UiChoice,
            _styles: StylesChoice,
        ) -> Result<()> {
            self.calls
                .borrow_mut()
                .push("apply_feature_template".to_string());
            Ok(())
        }

        fn apply_ui_integration(
            &self,
            _project_dir: &Path,
            _ui: UiChoice,
            _package_manager: PackageManager,
        ) -> Result<()> {
            self.calls
                .borrow_mut()
                .push("apply_ui_integration".to_string());
            Ok(())
        }
    }

    #[derive(Default)]
    struct FakeReporter;

    impl ProgressReporter for FakeReporter {
        //fn show_banner(&self) {}
        fn stage_start(&self, _stage: &str, _message: &str) {}
        fn stage_ok(&self, _stage: &str, _message: &str) {}
        fn stage_error(&self, _stage: &str, _message: &str) {}
        fn summary(&self, _project_name: &str, _project_dir: &Path, _options: ResolvedOptions) {}
    }

    #[test]
    fn execute_runs_expected_flow() {
        let env = FakeEnvironment {
            exists: false,
            cwd: PathBuf::from("/tmp"),
        };
        let ui_selector = FakeUiSelector {
            ui: UiChoice::None,
            styles: StylesChoice::Css,
        };
        let seeder = FakeSeeder::default();
        let reporter = FakeReporter;
        let use_case = NewProjectUseCase::new(&env, &ui_selector, &seeder, &reporter);

        use_case
            .execute(NewProjectRequest {
                project_name: "demo-app".to_string(),
                ui: None,
                styles: None,
                package_manager: Some(PackageManager::Npm),
                architecture: Some(ArchitectureProfile::Clean),
                skip_install: true,
                skip_git: false,
                yes: true,
            })
            .unwrap();

        assert_eq!(
            seeder.calls.borrow().clone(),
            vec![
                "ensure_required_tools",
                "scaffold_angular_project",
                "apply_architecture_template",
                "apply_feature_template",
                "apply_ui_integration"
            ]
        );
    }
}
