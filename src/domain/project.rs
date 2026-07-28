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
}

#[derive(Debug, Clone)]
pub struct GenerateFeatureRequest {
    pub project_dir: String,
    pub name: String,
    pub architecture: ArchitectureProfile,
    pub prefix: String,
    pub fields: Vec<String>,
}
