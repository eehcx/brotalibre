use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;

use crate::domain::project::ArchitectureProfile;
use crate::domain::project::AstroResolvedOptions;
use crate::domain::project::DocsEngine;
use crate::domain::project::Framework;
use crate::domain::project::PackageManager;
use crate::domain::project::ResolvedOptions;
use crate::domain::project::UiChoice;
use crate::domain::styles_choice::StylesChoice;

pub trait UiSelector {
    fn select_framework(&self) -> Result<Framework>;
    fn select_ui(&self) -> Result<UiChoice>;
    fn select_styles(&self) -> Result<StylesChoice>;
    fn select_package_manager(&self) -> Result<PackageManager>;
    fn select_architecture(&self) -> Result<ArchitectureProfile>;
    fn select_docs_engine(&self) -> Result<DocsEngine>;
    fn select_locales(&self) -> Result<Vec<String>>;
}

pub trait Environment {
    fn project_exists(&self, project_name: &str) -> bool;
    fn current_dir(&self) -> Result<PathBuf>;
    fn is_ci(&self) -> bool;
    fn is_interactive_terminal(&self) -> bool;
}

pub trait Seeder {
    fn ensure_required_tools(&self, package_manager: PackageManager) -> Result<()>;
    fn scaffold_angular_project(&self, project_name: &str, options: ResolvedOptions) -> Result<()>;
    fn apply_architecture_template(
        &self,
        project_dir: &Path,
        architecture: ArchitectureProfile,
        project_name: &str,
        styles: StylesChoice,
    ) -> Result<()>;
    #[allow(clippy::too_many_arguments)]
    fn apply_feature_template(
        &self,
        project_dir: &Path,
        architecture: ArchitectureProfile,
        name: &str,
        prefix: &str,
        fields: &[String],
        ui: UiChoice,
        styles: StylesChoice,
    ) -> Result<()>;
    fn apply_ui_integration(
        &self,
        project_dir: &Path,
        ui: UiChoice,
        package_manager: PackageManager,
    ) -> Result<()>;
}

/// Seeder port for the Astro docs i18n flow.
///
/// Kept as a separate trait from `Seeder` because the Angular and Astro
/// flows are orthogonal (they share neither the scaffolding command nor
/// the post-scaffold template application), so the Angular `Seeder` should
/// not have to know about Astro and vice versa. `SystemSeeder` implements
/// both; tests inject the one they need.
pub trait AstroSeeder {
    /// Ensure the tools required to scaffold an Astro project are available
    /// (node, the chosen package manager, npm/npx for `npm create astro`).
    fn ensure_astro_tools(&self, package_manager: PackageManager) -> Result<()>;

    /// Scaffold a fresh Astro project at `project_name` using the selected
    /// docs engine. Mirrors `scaffold_angular_project` but shells out to
    /// `npm create starlight@latest` / `npm create astro@latest`.
    fn scaffold_astro_project(
        &self,
        project_name: &str,
        options: &AstroResolvedOptions,
    ) -> Result<()>;

    /// Apply the Astro docs templates (config + per-locale content + i18n UI
    /// strings for the native engine) on top of the freshly scaffolded
    /// project at `project_dir`.
    fn apply_astro_template(
        &self,
        project_dir: &Path,
        docs_engine: DocsEngine,
        project_name: &str,
        locales: &[String],
    ) -> Result<()>;
}

pub trait ConfigWriter {
    fn write_config(
        &self,
        project_dir: &Path,
        config: &crate::domain::project_config::ProjectConfig,
    ) -> Result<()>;
}

pub trait ProgressReporter {
    //fn show_banner(&self);
    fn stage_start(&self, stage: &str, message: &str);
    fn stage_ok(&self, stage: &str, message: &str);
    fn stage_error(&self, stage: &str, message: &str);
    fn summary(&self, project_name: &str, project_dir: &Path, options: ResolvedOptions);
    fn astro_summary(&self, project_name: &str, project_dir: &Path, options: &AstroResolvedOptions);
}
