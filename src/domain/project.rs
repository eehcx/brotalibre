use crate::domain::profile::Profile;
use crate::domain::styles_choice::StylesChoice;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiChoice {
    Material,
    Primeng,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
    Bun,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchitectureProfile {
    Clean,
    Ddd,
}

/// Target frontend framework that `brota new` scaffolds.
///
/// `Angular` is the historical default and keeps the existing
/// ui/styles/architecture flow intact. `Astro` enables the docs
/// i18n flow with a selectable `DocsEngine`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Framework {
    #[default]
    Angular,
    Astro,
}

/// Documentation engine used when scaffolding Astro projects.
///
/// `Starlight` is the recommended default: the official Astro docs
/// theme ships with i18n, sidebar, search and dark mode out of the box.
/// `Native` uses Astro's native i18n routing + Content Collections and
/// gives full control over layout and UI strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DocsEngine {
    #[default]
    Starlight,
    Native,
}

#[derive(Debug, Clone, Copy)]
pub struct ResolvedOptions {
    pub ui: UiChoice,
    pub styles: StylesChoice,
    pub package_manager: PackageManager,
    pub architecture: ArchitectureProfile,
    pub skip_install: bool,
    pub skip_git: bool,
}

/// Resolved options for an Astro docs project.
///
/// This is the Astro counterpart of `ResolvedOptions`: the two flows are
/// orthogonal and share no fields beyond the package manager and the
/// `skip_install`/`skip_git` flags, so a dedicated type keeps each
/// framework's contract narrow and explicit.
#[derive(Debug, Clone)]
pub struct AstroResolvedOptions {
    pub docs_engine: DocsEngine,
    pub package_manager: PackageManager,
    /// Locales for i18n, e.g. `["en", "es"]`. The first entry is the
    /// default locale. Always non-empty after resolution (`["en"]` if the
    /// user did not pass `--i18n`).
    pub locales: Vec<String>,
    pub skip_install: bool,
    pub skip_git: bool,
}

#[derive(Debug, Clone)]
pub struct NewProjectRequest {
    pub project_name: String,
    pub ui: Option<UiChoice>,
    pub styles: Option<StylesChoice>,
    pub package_manager: Option<PackageManager>,
    pub architecture: Option<ArchitectureProfile>,
    pub skip_install: bool,
    pub skip_git: bool,
    pub yes: bool,
    /// Product profile that selects coherent defaults for framework,
    /// architecture, UI library, style engine, and package manager.
    /// When `None`, defaults to the `AngularAdmin` profile under `--yes`.
    pub profile: Option<Profile>,
    /// Target framework. When `None`, interactive runs ask the user and
    /// non-interactive/`--yes` runs keep Angular as the default.
    pub framework: Option<Framework>,
    /// Docs engine for Astro projects. Ignored when `framework` is Angular.
    /// When `None` it resolves to `DocsEngine::Starlight`.
    pub docs_engine: Option<DocsEngine>,
    /// Locales for Astro docs i18n, e.g. `["en", "es"]`. The first entry is
    /// the default locale. Ignored when `framework` is Angular.
    pub locales: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct GenerateFeatureRequest {
    pub project_dir: String,
    pub name: String,
    pub ui: Option<UiChoice>,
    pub styles: Option<StylesChoice>,
    pub architecture: ArchitectureProfile,
    pub prefix: String,
    pub fields: Vec<String>,
}
