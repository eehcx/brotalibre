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
    /// Target framework. When `None` it resolves to `Framework::Angular`
    /// (the historical behavior) unless `--yes` is used.
    #[allow(dead_code)]
    pub framework: Option<Framework>,
    /// Docs engine for Astro projects. Ignored when `framework` is Angular.
    /// When `None` it resolves to `DocsEngine::Starlight`.
    #[allow(dead_code)]
    pub docs_engine: Option<DocsEngine>,
    /// Locales for Astro docs i18n, e.g. `["en", "es"]`. The first entry is
    /// the default locale. Ignored when `framework` is Angular.
    #[allow(dead_code)]
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
