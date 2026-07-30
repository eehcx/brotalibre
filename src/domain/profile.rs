use crate::domain::project::ArchitectureProfile;
use crate::domain::project::DocsEngine;
use crate::domain::project::Framework;
use crate::domain::project::NewProjectRequest;
use crate::domain::project::PackageManager;
use crate::domain::project::UiChoice;
use crate::domain::styles_choice::StylesChoice;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Profile {
    #[default]
    AngularAdmin,
    AstroLanding,
    AstroDocs,
}

#[derive(Debug, Clone, Copy)]
pub struct ProfileDefaults {
    pub framework: Option<Framework>,
    pub architecture: Option<ArchitectureProfile>,
    pub ui: Option<UiChoice>,
    pub styles: Option<StylesChoice>,
    pub package_manager: Option<PackageManager>,
    pub docs_engine: Option<DocsEngine>,
}

impl Profile {
    pub fn defaults(self) -> ProfileDefaults {
        match self {
            Profile::AngularAdmin => ProfileDefaults {
                framework: Some(Framework::Angular),
                architecture: Some(ArchitectureProfile::Clean),
                ui: Some(UiChoice::Material),
                styles: Some(StylesChoice::Scss),
                package_manager: Some(PackageManager::Npm),
                docs_engine: None,
            },
            Profile::AstroLanding => ProfileDefaults {
                framework: Some(Framework::Astro),
                architecture: None,
                ui: None,
                styles: None,
                package_manager: Some(PackageManager::Npm),
                docs_engine: None,
            },
            Profile::AstroDocs => ProfileDefaults {
                framework: Some(Framework::Astro),
                architecture: None,
                ui: None,
                styles: None,
                package_manager: Some(PackageManager::Npm),
                docs_engine: Some(DocsEngine::Starlight),
            },
        }
    }

    /// Fill `None` fields in `request` with this profile's defaults.
    /// Explicit CLI flags (already `Some`) are never overridden.
    pub fn apply_to_request(self, request: &mut NewProjectRequest) {
        let d = self.defaults();
        if request.framework.is_none() {
            request.framework = d.framework;
        }
        if request.architecture.is_none() {
            request.architecture = d.architecture;
        }
        if request.ui.is_none() {
            request.ui = d.ui;
        }
        if request.styles.is_none() {
            request.styles = d.styles;
        }
        if request.package_manager.is_none() {
            request.package_manager = d.package_manager;
        }
        if request.docs_engine.is_none() {
            request.docs_engine = d.docs_engine;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angular_admin_defaults() {
        let d = Profile::AngularAdmin.defaults();
        assert_eq!(d.framework, Some(Framework::Angular));
        assert_eq!(d.architecture, Some(ArchitectureProfile::Clean));
        assert_eq!(d.ui, Some(UiChoice::Material));
        assert_eq!(d.styles, Some(StylesChoice::Scss));
        assert_eq!(d.package_manager, Some(PackageManager::Npm));
        assert_eq!(d.docs_engine, None);
    }

    #[test]
    fn astro_docs_defaults() {
        let d = Profile::AstroDocs.defaults();
        assert_eq!(d.framework, Some(Framework::Astro));
        assert_eq!(d.architecture, None);
        assert_eq!(d.ui, None);
        assert_eq!(d.styles, None);
        assert_eq!(d.package_manager, Some(PackageManager::Npm));
        assert_eq!(d.docs_engine, Some(DocsEngine::Starlight));
    }

    #[test]
    fn astro_landing_defaults() {
        let d = Profile::AstroLanding.defaults();
        assert_eq!(d.framework, Some(Framework::Astro));
        assert_eq!(d.architecture, None);
        assert_eq!(d.ui, None);
        assert_eq!(d.styles, None);
        assert_eq!(d.package_manager, Some(PackageManager::Npm));
        assert_eq!(d.docs_engine, None);
    }

    #[test]
    fn default_profile_is_angular_admin() {
        let p: Profile = Default::default();
        assert_eq!(p, Profile::AngularAdmin);
    }

    #[test]
    fn apply_angular_admin_profile_fills_none_fields() {
        use crate::domain::project::NewProjectRequest;

        let mut req = NewProjectRequest {
            project_name: "my-app".into(),
            profile: Some(Profile::AngularAdmin),
            ui: None,
            styles: None,
            package_manager: None,
            architecture: None,
            skip_install: false,
            skip_git: false,
            yes: true,
            framework: None,
            docs_engine: None,
            locales: vec![],
        };
        Profile::AngularAdmin.apply_to_request(&mut req);
        assert_eq!(req.framework, Some(Framework::Angular));
        assert_eq!(req.architecture, Some(ArchitectureProfile::Clean));
        assert_eq!(req.ui, Some(UiChoice::Material));
        assert_eq!(req.styles, Some(StylesChoice::Scss));
        assert_eq!(req.package_manager, Some(PackageManager::Npm));
        assert_eq!(req.docs_engine, None);
    }

    #[test]
    fn apply_profile_does_not_override_explicit_flags() {
        use crate::domain::project::NewProjectRequest;

        let mut req = NewProjectRequest {
            project_name: "my-app".into(),
            profile: Some(Profile::AngularAdmin),
            ui: Some(UiChoice::Primeng),
            styles: Some(StylesChoice::Css),
            package_manager: Some(PackageManager::Pnpm),
            architecture: Some(ArchitectureProfile::Ddd),
            skip_install: false,
            skip_git: false,
            yes: true,
            framework: Some(Framework::Astro),
            docs_engine: Some(DocsEngine::Native),
            locales: vec![],
        };
        Profile::AngularAdmin.apply_to_request(&mut req);
        // Explicit values must survive — profile only fills None
        assert_eq!(req.framework, Some(Framework::Astro));
        assert_eq!(req.architecture, Some(ArchitectureProfile::Ddd));
        assert_eq!(req.ui, Some(UiChoice::Primeng));
        assert_eq!(req.styles, Some(StylesChoice::Css));
        assert_eq!(req.package_manager, Some(PackageManager::Pnpm));
        assert_eq!(req.docs_engine, Some(DocsEngine::Native));
    }

    #[test]
    fn apply_astro_docs_profile_sets_starlight() {
        use crate::domain::project::NewProjectRequest;

        let mut req = NewProjectRequest {
            project_name: "docs".into(),
            profile: Some(Profile::AstroDocs),
            ui: None,
            styles: None,
            package_manager: None,
            architecture: None,
            skip_install: false,
            skip_git: false,
            yes: true,
            framework: None,
            docs_engine: None,
            locales: vec![],
        };
        Profile::AstroDocs.apply_to_request(&mut req);
        assert_eq!(req.framework, Some(Framework::Astro));
        assert_eq!(req.docs_engine, Some(DocsEngine::Starlight));
        assert_eq!(req.architecture, None);
        assert_eq!(req.ui, None);
    }
}
