use crate::domain::profile::Profile;
use crate::domain::project::ArchitectureProfile;
use crate::domain::project::AstroResolvedOptions;
use crate::domain::project::Framework;
use crate::domain::project::PackageManager;
use crate::domain::project::ResolvedOptions;
use crate::domain::project::UiChoice;
use crate::domain::styles_choice::StylesChoice;

pub const BROTA_CONFIG_FILENAME: &str = "brota.yaml";
pub const BROTA_CONFIG_SCHEMA_VERSION: &str = "1";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    pub schema_version: String,
    pub project: ProjectMeta,
    pub profile: String,
    pub target: Target,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Application>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<Ui>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectMeta {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub framework: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    pub package_manager: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub initial_route: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ui {
    pub library: String,
    pub style_engine: String,
}

fn profile_to_config_string(p: Profile) -> String {
    match p {
        Profile::AngularAdmin => "angular-admin".into(),
        Profile::AstroLanding => "astro-landing".into(),
        Profile::AstroDocs => "astro-docs".into(),
    }
}

fn framework_to_config_string(f: Framework) -> String {
    match f {
        Framework::Angular => "angular".into(),
        Framework::Astro => "astro".into(),
    }
}

fn architecture_to_config_string(a: ArchitectureProfile) -> String {
    match a {
        ArchitectureProfile::Clean => "feature-clean".into(),
        ArchitectureProfile::Ddd => "ddd".into(),
    }
}

fn package_manager_to_config_string(p: PackageManager) -> String {
    match p {
        PackageManager::Npm => "npm".into(),
        PackageManager::Pnpm => "pnpm".into(),
        PackageManager::Yarn => "yarn".into(),
        PackageManager::Bun => "bun".into(),
    }
}

fn ui_choice_to_config_string(u: UiChoice) -> String {
    match u {
        UiChoice::Material => "material".into(),
        UiChoice::Primeng => "primeng".into(),
        UiChoice::None => "native".into(),
    }
}

fn styles_choice_to_config_string(s: StylesChoice) -> String {
    match s {
        StylesChoice::Css => "css".into(),
        StylesChoice::Scss => "scss".into(),
        StylesChoice::Sass => "sass".into(),
        StylesChoice::Less => "less".into(),
        StylesChoice::TailwindCSS => "tailwindcss".into(),
    }
}

impl ProjectConfig {
    pub fn from_resolved(project_name: &str, profile: Profile, options: &ResolvedOptions) -> Self {
        ProjectConfig {
            schema_version: BROTA_CONFIG_SCHEMA_VERSION.to_string(),
            project: ProjectMeta {
                name: project_name.to_string(),
                locale: None,
            },
            profile: profile_to_config_string(profile),
            target: Target {
                framework: framework_to_config_string(Framework::Angular),
                architecture: Some(architecture_to_config_string(options.architecture)),
                package_manager: package_manager_to_config_string(options.package_manager),
            },
            application: Some(Application {
                initial_route: "welcome".into(),
            }),
            ui: Some(Ui {
                library: ui_choice_to_config_string(options.ui),
                style_engine: styles_choice_to_config_string(options.styles),
            }),
        }
    }

    pub fn from_astro_resolved(
        project_name: &str,
        profile: Profile,
        options: &AstroResolvedOptions,
    ) -> Self {
        ProjectConfig {
            schema_version: BROTA_CONFIG_SCHEMA_VERSION.to_string(),
            project: ProjectMeta {
                name: project_name.to_string(),
                locale: options.locales.first().cloned(),
            },
            profile: profile_to_config_string(profile),
            target: Target {
                framework: framework_to_config_string(Framework::Astro),
                architecture: None,
                package_manager: package_manager_to_config_string(options.package_manager),
            },
            application: None,
            ui: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::project::ArchitectureProfile;
    use crate::domain::project::AstroResolvedOptions;
    use crate::domain::project::ResolvedOptions;

    fn angular_options() -> ResolvedOptions {
        ResolvedOptions {
            ui: UiChoice::Material,
            styles: StylesChoice::Scss,
            package_manager: PackageManager::Pnpm,
            architecture: ArchitectureProfile::Clean,
            skip_install: true,
            skip_git: false,
        }
    }

    #[test]
    fn writes_and_reads_brota_yaml_from_filesystem() {
        use std::fs;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let config =
            ProjectConfig::from_resolved("fs-test", Profile::AngularAdmin, &angular_options());

        let path = dir.path().join(BROTA_CONFIG_FILENAME);
        let yaml = serde_yaml_ng::to_string(&config).unwrap();
        fs::write(&path, &yaml).unwrap();

        let loaded_yaml = fs::read_to_string(&path).unwrap();
        let loaded: ProjectConfig = serde_yaml_ng::from_str(&loaded_yaml).unwrap();

        assert_eq!(loaded.project.name, "fs-test");
        assert_eq!(loaded.profile, "angular-admin");
        assert_eq!(loaded.target.framework, "angular");
        assert!(loaded.ui.is_some());
    }

    #[test]
    fn builds_correct_angular_project_config() {
        let config = ProjectConfig::from_resolved(
            "inventory-console",
            Profile::AngularAdmin,
            &angular_options(),
        );

        assert_eq!(config.schema_version, "1");
        assert_eq!(config.project.name, "inventory-console");
        assert_eq!(config.project.locale, None);
        assert_eq!(config.profile, "angular-admin");
        assert_eq!(config.target.framework, "angular");
        assert_eq!(config.target.architecture.as_deref(), Some("feature-clean"));
        assert_eq!(config.target.package_manager, "pnpm");
        assert_eq!(
            config.application.as_ref().unwrap().initial_route,
            "welcome"
        );
        assert_eq!(config.ui.as_ref().unwrap().library, "material");
        assert_eq!(config.ui.as_ref().unwrap().style_engine, "scss");
    }

    #[test]
    fn serializes_ui_none_as_native() {
        let options = ResolvedOptions {
            ui: UiChoice::None,
            styles: StylesChoice::Css,
            ..angular_options()
        };
        let config = ProjectConfig::from_resolved("plain-app", Profile::AngularAdmin, &options);

        assert_eq!(config.ui.as_ref().unwrap().library, "native");
        assert_eq!(config.ui.as_ref().unwrap().style_engine, "css");
    }

    #[test]
    fn builds_correct_astro_project_config_with_locale() {
        let options = AstroResolvedOptions {
            docs_engine: crate::domain::project::DocsEngine::Starlight,
            package_manager: PackageManager::Npm,
            locales: vec!["es".to_string(), "en".to_string()],
            skip_install: true,
            skip_git: false,
        };
        let config = ProjectConfig::from_astro_resolved("my-docs", Profile::AstroDocs, &options);

        assert_eq!(config.schema_version, "1");
        assert_eq!(config.project.name, "my-docs");
        assert_eq!(config.project.locale.as_deref(), Some("es"));
        assert_eq!(config.profile, "astro-docs");
        assert_eq!(config.target.framework, "astro");
        assert_eq!(config.target.architecture, None);
        assert_eq!(config.target.package_manager, "npm");
        assert!(config.application.is_none());
        assert!(config.ui.is_none());
    }

    #[test]
    fn astro_no_locale_sets_none() {
        let options = AstroResolvedOptions {
            docs_engine: crate::domain::project::DocsEngine::Native,
            package_manager: PackageManager::Pnpm,
            locales: vec![],
            skip_install: false,
            skip_git: true,
        };
        let config = ProjectConfig::from_astro_resolved("minimal", Profile::AstroLanding, &options);

        assert_eq!(config.project.locale, None);
        assert_eq!(config.profile, "astro-landing");
    }

    #[test]
    fn packages_are_mapped_correctly() {
        let check = |pm: PackageManager, expected: &str| {
            let options = ResolvedOptions {
                package_manager: pm,
                ..angular_options()
            };
            let config = ProjectConfig::from_resolved("pkg-test", Profile::AngularAdmin, &options);
            assert_eq!(config.target.package_manager, expected);
        };
        check(PackageManager::Npm, "npm");
        check(PackageManager::Pnpm, "pnpm");
        check(PackageManager::Yarn, "yarn");
        check(PackageManager::Bun, "bun");
    }

    #[test]
    fn profile_enum_all_variants() {
        let opts = &angular_options();
        let r1 = ProjectConfig::from_resolved("a", Profile::AngularAdmin, opts);
        assert_eq!(r1.profile, "angular-admin");
        let r2 = ProjectConfig::from_resolved("b", Profile::AstroLanding, opts);
        assert_eq!(r2.profile, "astro-landing");
        let r3 = ProjectConfig::from_resolved("c", Profile::AstroDocs, opts);
        assert_eq!(r3.profile, "astro-docs");
    }

    #[test]
    fn serde_roundtrip_angular() {
        let config =
            ProjectConfig::from_resolved("roundtrip", Profile::AngularAdmin, &angular_options());
        let yaml = serde_yaml_ng::to_string(&config).unwrap();
        let deserialized: ProjectConfig = serde_yaml_ng::from_str(&yaml).unwrap();
        assert_eq!(deserialized.schema_version, "1");
        assert_eq!(deserialized.project.name, "roundtrip");
        assert_eq!(deserialized.target.framework, "angular");
        assert_eq!(deserialized.ui.as_ref().unwrap().library, "material");
    }

    #[test]
    fn serde_roundtrip_astro() {
        let options = AstroResolvedOptions {
            docs_engine: crate::domain::project::DocsEngine::Starlight,
            package_manager: PackageManager::Npm,
            locales: vec!["es".to_string()],
            skip_install: true,
            skip_git: true,
        };
        let config =
            ProjectConfig::from_astro_resolved("astro-roundtrip", Profile::AstroDocs, &options);
        let yaml = serde_yaml_ng::to_string(&config).unwrap();
        let deserialized: ProjectConfig = serde_yaml_ng::from_str(&yaml).unwrap();
        assert_eq!(deserialized.project.name, "astro-roundtrip");
        assert_eq!(deserialized.profile, "astro-docs");
        assert!(deserialized.ui.is_none());
    }
}
