use std::path::Path;

use anyhow::{Result, bail};

use crate::application::ports::{ConfigReader, ProgressReporter, Seeder};
use crate::domain::project::{ArchitectureProfile, GenerateFeatureRequest, UiChoice};
use crate::domain::project_config::ProjectConfig;
use crate::domain::styles_choice::StylesChoice;

pub struct GenerateFeatureUseCase<'a> {
    seeder: &'a dyn Seeder,
    reporter: &'a dyn ProgressReporter,
    config_reader: &'a dyn ConfigReader,
}

impl<'a> GenerateFeatureUseCase<'a> {
    pub fn new(
        seeder: &'a dyn Seeder,
        reporter: &'a dyn ProgressReporter,
        config_reader: &'a dyn ConfigReader,
    ) -> Self {
        Self {
            seeder,
            reporter,
            config_reader,
        }
    }

    pub fn execute(&self, request: GenerateFeatureRequest) -> Result<()> {
        let project_dir = Path::new(&request.project_dir);
        let config = self.config_reader.read_config(project_dir)?;
        let configured = resolve_feature_options(&config)?;

        self.reporter
            .stage_start("generate", &format!("generating {} feature", request.name));

        self.seeder.apply_feature_template(
            project_dir,
            configured.architecture,
            &request.name,
            &request.prefix,
            &request.fields,
            configured.ui,
            configured.styles,
        )?;

        self.reporter
            .stage_ok("generate", &format!("feature {} created", request.name));

        Ok(())
    }
}

struct FeatureOptions {
    architecture: ArchitectureProfile,
    ui: UiChoice,
    styles: StylesChoice,
}

fn resolve_feature_options(config: &ProjectConfig) -> Result<FeatureOptions> {
    if config.schema_version != "1" {
        bail!(
            "unsupported brota.yaml schema version `{}`; expected `1`",
            config.schema_version
        );
    }

    if config.profile != "angular-admin" {
        bail!(
            "generate feature only supports profile `angular-admin`, found `{}`",
            config.profile
        );
    }

    if config.target.framework != "angular" {
        bail!(
            "generate feature only supports target framework `angular`, found `{}`",
            config.target.framework
        );
    }

    let architecture = match config.target.architecture.as_deref() {
        Some("feature-clean") => ArchitectureProfile::Clean,
        Some("ddd") => ArchitectureProfile::Ddd,
        Some(value) => bail!("unsupported target architecture `{value}`"),
        None => bail!("brota.yaml is missing `target.architecture`"),
    };

    let ui = match config.ui.as_ref().map(|ui| ui.library.as_str()) {
        Some("material") => UiChoice::Material,
        Some("primeng") => UiChoice::Primeng,
        Some("native") => UiChoice::None,
        Some(value) => bail!("unsupported UI library `{value}`"),
        None => bail!("brota.yaml is missing `ui.library`"),
    };

    let styles = match config.ui.as_ref().map(|ui| ui.style_engine.as_str()) {
        Some("css") => StylesChoice::Css,
        Some("scss") => StylesChoice::Scss,
        Some("sass") => StylesChoice::Sass,
        Some("less") => StylesChoice::Less,
        Some("tailwindcss") => StylesChoice::TailwindCSS,
        Some(value) => bail!("unsupported style engine `{value}`"),
        None => bail!("brota.yaml is missing `ui.styleEngine`"),
    };

    Ok(FeatureOptions {
        architecture,
        ui,
        styles,
    })
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::path::Path;

    use super::*;
    use crate::application::ports::{ConfigReader, ProgressReporter, Seeder};
    use crate::domain::project::{AstroResolvedOptions, PackageManager, ResolvedOptions};

    struct FakeConfigReader {
        config: Result<ProjectConfig, String>,
    }

    impl ConfigReader for FakeConfigReader {
        fn read_config(&self, _project_dir: &Path) -> Result<ProjectConfig> {
            self.config.clone().map_err(anyhow::Error::msg)
        }
    }

    #[derive(Default)]
    struct FakeSeeder {
        call: RefCell<Option<(ArchitectureProfile, UiChoice, StylesChoice)>>,
    }

    impl Seeder for FakeSeeder {
        fn ensure_required_tools(&self, _package_manager: PackageManager) -> Result<()> {
            Ok(())
        }

        fn scaffold_angular_project(
            &self,
            _project_name: &str,
            _options: ResolvedOptions,
        ) -> Result<()> {
            Ok(())
        }

        fn apply_architecture_template(
            &self,
            _project_dir: &Path,
            _architecture: ArchitectureProfile,
            _project_name: &str,
            _styles: StylesChoice,
        ) -> Result<()> {
            Ok(())
        }

        fn apply_feature_template(
            &self,
            _project_dir: &Path,
            architecture: ArchitectureProfile,
            _name: &str,
            _prefix: &str,
            _fields: &[String],
            ui: UiChoice,
            styles: StylesChoice,
        ) -> Result<()> {
            *self.call.borrow_mut() = Some((architecture, ui, styles));
            Ok(())
        }

        fn apply_ui_integration(
            &self,
            _project_dir: &Path,
            _ui: UiChoice,
            _package_manager: PackageManager,
        ) -> Result<()> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct FakeReporter;

    impl ProgressReporter for FakeReporter {
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

    fn config() -> ProjectConfig {
        ProjectConfig {
            schema_version: "1".into(),
            project: crate::domain::project_config::ProjectMeta {
                name: "demo".into(),
                locale: None,
            },
            profile: "angular-admin".into(),
            target: crate::domain::project_config::Target {
                framework: "angular".into(),
                architecture: Some("feature-clean".into()),
                package_manager: "npm".into(),
            },
            application: None,
            ui: Some(crate::domain::project_config::Ui {
                library: "material".into(),
                style_engine: "scss".into(),
            }),
        }
    }

    fn request() -> GenerateFeatureRequest {
        GenerateFeatureRequest {
            project_dir: ".".into(),
            name: "users".into(),
            prefix: "api".into(),
            fields: vec![],
        }
    }

    #[test]
    fn uses_project_config_as_the_only_generation_source() {
        let reader = FakeConfigReader {
            config: Ok(config()),
        };
        let seeder = FakeSeeder::default();
        let reporter = FakeReporter;
        let use_case = GenerateFeatureUseCase::new(&seeder, &reporter, &reader);

        use_case.execute(request()).unwrap();

        assert_eq!(
            *seeder.call.borrow(),
            Some((
                ArchitectureProfile::Clean,
                UiChoice::Material,
                StylesChoice::Scss
            ))
        );
    }

    #[test]
    fn rejects_non_angular_project() {
        let mut config = config();
        config.profile = "astro-docs".into();
        let reader = FakeConfigReader { config: Ok(config) };
        let seeder = FakeSeeder::default();
        let reporter = FakeReporter;
        let use_case = GenerateFeatureUseCase::new(&seeder, &reporter, &reader);

        let error = use_case.execute(request()).unwrap_err().to_string();

        assert!(error.contains("angular-admin"));
    }

    #[test]
    fn rejects_missing_config() {
        let reader = FakeConfigReader {
            config: Err("could not read brota.yaml".into()),
        };
        let seeder = FakeSeeder::default();
        let reporter = FakeReporter;
        let use_case = GenerateFeatureUseCase::new(&seeder, &reporter, &reader);

        let error = use_case.execute(request()).unwrap_err().to_string();

        assert!(error.contains("could not read brota.yaml"));
    }

    #[test]
    fn rejects_unknown_ui_library() {
        let mut config = config();
        config.ui.as_mut().unwrap().library = "unknown".into();
        let reader = FakeConfigReader { config: Ok(config) };
        let seeder = FakeSeeder::default();
        let reporter = FakeReporter;
        let use_case = GenerateFeatureUseCase::new(&seeder, &reporter, &reader);

        let error = use_case.execute(request()).unwrap_err().to_string();

        assert!(error.contains("unsupported UI library"));
    }

    #[test]
    fn rejects_unknown_schema_version() {
        let mut config = config();
        config.schema_version = "2".into();
        let reader = FakeConfigReader { config: Ok(config) };
        let seeder = FakeSeeder::default();
        let reporter = FakeReporter;
        let use_case = GenerateFeatureUseCase::new(&seeder, &reporter, &reader);

        let error = use_case.execute(request()).unwrap_err().to_string();

        assert!(error.contains("unsupported brota.yaml schema version"));
    }
}
