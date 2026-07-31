use anyhow::{Result, bail};

use crate::application::ports::AstroSeeder;
use crate::application::ports::Environment;
use crate::application::ports::ProgressReporter;
use crate::application::ports::Seeder;
use crate::application::ports::UiSelector;
use crate::domain::profile::Profile;
use crate::domain::project::ArchitectureProfile;
use crate::domain::project::AstroResolvedOptions;
use crate::domain::project::DocsEngine;
use crate::domain::project::Framework;
use crate::domain::project::NewProjectRequest;
use crate::domain::project::PackageManager;
use crate::domain::project::ResolvedOptions;
use crate::domain::project::UiChoice;
use crate::domain::styles_choice::StylesChoice;

pub struct NewProjectUseCase<'a> {
    env: &'a dyn Environment,
    ui_selector: &'a dyn UiSelector,
    seeder: &'a dyn Seeder,
    astro_seeder: &'a dyn AstroSeeder,
    reporter: &'a dyn ProgressReporter,
}

impl<'a> NewProjectUseCase<'a> {
    pub fn new(
        env: &'a dyn Environment,
        ui_selector: &'a dyn UiSelector,
        seeder: &'a dyn Seeder,
        astro_seeder: &'a dyn AstroSeeder,
        reporter: &'a dyn ProgressReporter,
    ) -> Self {
        Self {
            env,
            ui_selector,
            seeder,
            astro_seeder,
            reporter,
        }
    }

    pub fn execute(&self, mut request: NewProjectRequest) -> Result<()> {
        if let Some(profile) = request.profile {
            profile.apply_to_request(&mut request);
        } else if request.yes {
            Profile::AngularAdmin.apply_to_request(&mut request);
        }

        let framework = self.resolve_framework(&request)?;

        match framework {
            Framework::Astro => self.execute_astro(&request),
            Framework::Angular => self.execute_angular(&request),
        }
    }

    fn execute_angular(&self, request: &NewProjectRequest) -> Result<()> {
        let options = self.resolve_angular_options(request)?;

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

    fn execute_astro(&self, request: &NewProjectRequest) -> Result<()> {
        let options = self.resolve_astro_options(request)?;

        self.reporter
            .stage_start("preflight", "checking required tools");
        if let Err(err) = self
            .astro_seeder
            .ensure_astro_tools(options.package_manager)
        {
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

        let engine_label = match options.docs_engine {
            DocsEngine::Starlight => "Starlight",
            DocsEngine::Native => "Astro native",
        };

        self.reporter
            .stage_start("scaffold", &format!("creating {engine_label} project"));
        if let Err(err) = self
            .astro_seeder
            .scaffold_astro_project(&request.project_name, &options)
        {
            self.reporter
                .stage_error("scaffold", "Astro scaffolding failed");
            return Err(err);
        }
        self.reporter
            .stage_ok("scaffold", &format!("{engine_label} project created"));

        let absolute_project_dir = self.env.current_dir()?.join(&request.project_name);

        self.reporter.stage_start(
            "template",
            &format!("applying {engine_label} i18n template"),
        );
        if let Err(err) = self.astro_seeder.apply_astro_template(
            &absolute_project_dir,
            options.docs_engine,
            &request.project_name,
            &options.locales,
        ) {
            self.reporter
                .stage_error("template", "Astro template setup failed");
            return Err(err);
        }
        self.reporter.stage_ok("template", "i18n template applied");

        self.reporter
            .astro_summary(&request.project_name, &absolute_project_dir, &options);

        Ok(())
    }

    fn resolve_framework(&self, request: &NewProjectRequest) -> Result<Framework> {
        if let Some(framework) = request.framework {
            return Ok(framework);
        }
        if request.yes || self.env.is_ci() || !self.env.is_interactive_terminal() {
            Ok(Framework::Angular)
        } else {
            self.ui_selector.select_framework()
        }
    }

    fn resolve_angular_options(&self, request: &NewProjectRequest) -> Result<ResolvedOptions> {
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

    fn resolve_astro_options(&self, request: &NewProjectRequest) -> Result<AstroResolvedOptions> {
        let package_manager = if let Some(value) = request.package_manager {
            value
        } else if request.yes {
            PackageManager::Npm
        } else {
            self.ui_selector.select_package_manager()?
        };

        let docs_engine = if let Some(docs_engine) = request.docs_engine {
            docs_engine
        } else if request.yes || self.env.is_ci() || !self.env.is_interactive_terminal() {
            DocsEngine::Starlight
        } else {
            self.ui_selector.select_docs_engine()?
        };

        // Locales: explicit CLI list wins; otherwise default to a single
        // English locale so a `--yes` scaffold produces a working site.
        let locales = if !request.locales.is_empty() {
            request.locales.clone()
        } else if request.yes || self.env.is_ci() || !self.env.is_interactive_terminal() {
            vec!["en".to_string()]
        } else {
            self.ui_selector.select_locales()?
        };

        Ok(AstroResolvedOptions {
            docs_engine,
            package_manager,
            locales,
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
    use crate::application::ports::AstroSeeder;
    use crate::application::ports::Environment;
    use crate::application::ports::ProgressReporter;
    use crate::application::ports::Seeder;
    use crate::application::ports::UiSelector;
    use crate::domain::project::DocsEngine;
    use crate::domain::project::NewProjectRequest;
    use crate::domain::project::PackageManager;

    struct FakeUiSelector {
        framework: Framework,
        ui: UiChoice,
        styles: StylesChoice,
        docs_engine: DocsEngine,
        locales: Vec<String>,
    }

    impl UiSelector for FakeUiSelector {
        fn select_framework(&self) -> Result<Framework> {
            Ok(self.framework)
        }

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

        fn select_docs_engine(&self) -> Result<DocsEngine> {
            Ok(self.docs_engine)
        }

        fn select_locales(&self) -> Result<Vec<String>> {
            Ok(self.locales.clone())
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
        resolved_ui: RefCell<Option<UiChoice>>,
        resolved_styles: RefCell<Option<StylesChoice>>,
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
            options: ResolvedOptions,
        ) -> Result<()> {
            self.resolved_ui.borrow_mut().replace(options.ui);
            self.resolved_styles.borrow_mut().replace(options.styles);
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
    struct FakeAstroSeeder {
        calls: RefCell<Vec<String>>,
    }

    impl AstroSeeder for FakeAstroSeeder {
        fn ensure_astro_tools(&self, _package_manager: PackageManager) -> Result<()> {
            self.calls
                .borrow_mut()
                .push("ensure_astro_tools".to_string());
            Ok(())
        }

        fn scaffold_astro_project(
            &self,
            project_name: &str,
            options: &AstroResolvedOptions,
        ) -> Result<()> {
            self.calls.borrow_mut().push(format!(
                "scaffold_astro_project:{}:{:?}",
                project_name, options.docs_engine
            ));
            Ok(())
        }

        fn apply_astro_template(
            &self,
            _project_dir: &Path,
            docs_engine: DocsEngine,
            project_name: &str,
            locales: &[String],
        ) -> Result<()> {
            self.calls.borrow_mut().push(format!(
                "apply_astro_template:{:?}:{}:{}",
                docs_engine,
                project_name,
                locales.join(",")
            ));
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
        fn astro_summary(
            &self,
            _project_name: &str,
            _project_dir: &Path,
            _options: &AstroResolvedOptions,
        ) {
        }
    }

    fn make_request(
        project_name: &str,
        framework: Option<Framework>,
        docs_engine: Option<DocsEngine>,
        locales: Vec<String>,
    ) -> NewProjectRequest {
        NewProjectRequest {
            project_name: project_name.to_string(),
            profile: None,
            ui: None,
            styles: None,
            package_manager: Some(PackageManager::Npm),
            architecture: Some(ArchitectureProfile::Clean),
            skip_install: true,
            skip_git: false,
            yes: true,
            framework,
            docs_engine,
            locales,
        }
    }

    #[test]
    fn execute_runs_expected_angular_flow() {
        let env = FakeEnvironment {
            exists: false,
            cwd: PathBuf::from("/tmp"),
        };
        let ui_selector = FakeUiSelector {
            framework: Framework::Angular,
            ui: UiChoice::None,
            styles: StylesChoice::Css,
            docs_engine: DocsEngine::Starlight,
            locales: vec!["en".to_string()],
        };
        let seeder = FakeSeeder::default();
        let astro_seeder = FakeAstroSeeder::default();
        let reporter = FakeReporter;
        let use_case =
            NewProjectUseCase::new(&env, &ui_selector, &seeder, &astro_seeder, &reporter);

        use_case
            .execute(make_request("demo-app", None, None, vec![]))
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
        // The Angular flow must never touch the Astro seeder.
        assert!(astro_seeder.calls.borrow().is_empty());
    }

    #[test]
    fn interactive_framework_selection_enters_astro_docs_flow() {
        let env = FakeEnvironment {
            exists: false,
            cwd: PathBuf::from("/tmp"),
        };
        let ui_selector = FakeUiSelector {
            framework: Framework::Astro,
            ui: UiChoice::None,
            styles: StylesChoice::Css,
            docs_engine: DocsEngine::Native,
            locales: vec!["en".to_string(), "es".to_string()],
        };
        let seeder = FakeSeeder::default();
        let astro_seeder = FakeAstroSeeder::default();
        let reporter = FakeReporter;
        let use_case =
            NewProjectUseCase::new(&env, &ui_selector, &seeder, &astro_seeder, &reporter);

        use_case
            .execute(NewProjectRequest {
                project_name: "interactive-docs".to_string(),
                profile: None,
                ui: None,
                styles: None,
                package_manager: None,
                architecture: None,
                skip_install: true,
                skip_git: true,
                yes: false,
                framework: None,
                docs_engine: None,
                locales: vec![],
            })
            .unwrap();

        assert_eq!(
            astro_seeder.calls.borrow().clone(),
            vec![
                "ensure_astro_tools",
                "scaffold_astro_project:interactive-docs:Native",
                "apply_astro_template:Native:interactive-docs:en,es",
            ]
        );
        assert!(seeder.calls.borrow().is_empty());
    }

    #[test]
    fn execute_astro_starlight_with_default_locale() {
        let env = FakeEnvironment {
            exists: false,
            cwd: PathBuf::from("/tmp"),
        };
        let ui_selector = FakeUiSelector {
            framework: Framework::Astro,
            ui: UiChoice::None,
            styles: StylesChoice::Css,
            docs_engine: DocsEngine::Starlight,
            locales: vec!["en".to_string()],
        };
        let seeder = FakeSeeder::default();
        let astro_seeder = FakeAstroSeeder::default();
        let reporter = FakeReporter;
        let use_case =
            NewProjectUseCase::new(&env, &ui_selector, &seeder, &astro_seeder, &reporter);

        use_case
            .execute(make_request(
                "my-docs",
                Some(Framework::Astro),
                Some(DocsEngine::Starlight),
                vec![],
            ))
            .unwrap();

        let calls = astro_seeder.calls.borrow().clone();
        assert_eq!(
            calls,
            vec![
                "ensure_astro_tools".to_string(),
                "scaffold_astro_project:my-docs:Starlight".to_string(),
                "apply_astro_template:Starlight:my-docs:en".to_string(),
            ]
        );
        // The Astro flow must never touch the Angular seeder.
        assert!(seeder.calls.borrow().is_empty());
    }

    #[test]
    fn execute_astro_native_with_explicit_locales() {
        let env = FakeEnvironment {
            exists: false,
            cwd: PathBuf::from("/tmp"),
        };
        let ui_selector = FakeUiSelector {
            framework: Framework::Astro,
            ui: UiChoice::None,
            styles: StylesChoice::Css,
            docs_engine: DocsEngine::Native,
            locales: vec!["en".to_string(), "es".to_string()],
        };
        let seeder = FakeSeeder::default();
        let astro_seeder = FakeAstroSeeder::default();
        let reporter = FakeReporter;
        let use_case =
            NewProjectUseCase::new(&env, &ui_selector, &seeder, &astro_seeder, &reporter);

        use_case
            .execute(make_request(
                "docs-site",
                Some(Framework::Astro),
                Some(DocsEngine::Native),
                vec!["en".to_string(), "es".to_string()],
            ))
            .unwrap();

        let calls = astro_seeder.calls.borrow().clone();
        assert_eq!(
            calls,
            vec![
                "ensure_astro_tools".to_string(),
                "scaffold_astro_project:docs-site:Native".to_string(),
                "apply_astro_template:Native:docs-site:en,es".to_string(),
            ]
        );
    }

    #[test]
    fn execute_astro_defaults_engine_to_starlight_when_unspecified() {
        let env = FakeEnvironment {
            exists: false,
            cwd: PathBuf::from("/tmp"),
        };
        let ui_selector = FakeUiSelector {
            framework: Framework::Astro,
            ui: UiChoice::None,
            styles: StylesChoice::Css,
            docs_engine: DocsEngine::Starlight,
            locales: vec!["en".to_string()],
        };
        let seeder = FakeSeeder::default();
        let astro_seeder = FakeAstroSeeder::default();
        let reporter = FakeReporter;
        let use_case =
            NewProjectUseCase::new(&env, &ui_selector, &seeder, &astro_seeder, &reporter);

        use_case
            .execute(make_request(
                "fallback-docs",
                Some(Framework::Astro),
                None,
                vec![],
            ))
            .unwrap();

        // Engine falls back to Starlight; locale falls back to "en".
        let calls = astro_seeder.calls.borrow().clone();
        assert!(
            calls
                .iter()
                .any(|c| c == "scaffold_astro_project:fallback-docs:Starlight")
        );
        assert!(
            calls
                .iter()
                .any(|c| c == "apply_astro_template:Starlight:fallback-docs:en")
        );
    }

    #[test]
    fn yes_without_profile_applies_angular_admin_defaults() {
        let env = FakeEnvironment {
            exists: false,
            cwd: PathBuf::from("/tmp"),
        };
        let ui_selector = FakeUiSelector {
            framework: Framework::Angular,
            ui: UiChoice::None,
            styles: StylesChoice::Css,
            docs_engine: DocsEngine::Starlight,
            locales: vec!["en".to_string()],
        };
        let seeder = FakeSeeder::default();
        let astro_seeder = FakeAstroSeeder::default();
        let reporter = FakeReporter;
        let use_case =
            NewProjectUseCase::new(&env, &ui_selector, &seeder, &astro_seeder, &reporter);

        // Request: yes=true, profile=None → should apply AngularAdmin defaults
        use_case
            .execute(make_request("material-app", None, None, vec![]))
            .unwrap();

        assert_eq!(*seeder.resolved_ui.borrow(), Some(UiChoice::Material));
        assert_eq!(*seeder.resolved_styles.borrow(), Some(StylesChoice::Scss));
    }

    #[test]
    fn explicit_ui_flag_overrides_profile_default() {
        let env = FakeEnvironment {
            exists: false,
            cwd: PathBuf::from("/tmp"),
        };
        let ui_selector = FakeUiSelector {
            framework: Framework::Angular,
            ui: UiChoice::None,
            styles: StylesChoice::Css,
            docs_engine: DocsEngine::Starlight,
            locales: vec!["en".to_string()],
        };
        let seeder = FakeSeeder::default();
        let astro_seeder = FakeAstroSeeder::default();
        let reporter = FakeReporter;
        let use_case =
            NewProjectUseCase::new(&env, &ui_selector, &seeder, &astro_seeder, &reporter);

        // Request: yes=true, profile=None, but explicit ui→Primeng, styles→Css
        let mut req = make_request("primeng-app", None, None, vec![]);
        req.ui = Some(UiChoice::Primeng);
        req.styles = Some(StylesChoice::Css);
        use_case.execute(req).unwrap();

        assert_eq!(*seeder.resolved_ui.borrow(), Some(UiChoice::Primeng));
        assert_eq!(*seeder.resolved_styles.borrow(), Some(StylesChoice::Css));
    }
}
