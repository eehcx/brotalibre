#![allow(dead_code)]

use crate::domain::project::PackageManager;
use crate::domain::project::UiChoice;

pub struct TuiInput {
    pub project_name: Option<String>,
    pub package_manager: Option<PackageManager>,
    pub ui_choice: Option<UiChoice>,
    pub styles_choice: Option<StylesChoice>,
    pub skip_install: bool,
}

impl TuiInput {
    pub fn new() -> Self {
        Self {
            project_name: None,
            package_manager: None,
            ui_choice: None,
            styles_choice: None,
            skip_install: false,
        }
    }

    pub fn with_project_name(mut self, name: String) -> Self {
        self.project_name = Some(name);
        self
    }

    pub fn with_package_manager(mut self, pm: PackageManager) -> Self {
        self.package_manager = Some(pm);
        self
    }

    pub fn with_ui_choice(mut self, ui: UiChoice) -> Self {
        self.ui_choice = Some(ui);
        self
    }

    pub fn with_styles_choice(mut self, styles: StylesChoice) -> Self {
        self.styles_choice = Some(styles);
        self
    }

    pub fn with_skip_install(mut self, skip: bool) -> Self {
        self.skip_install = skip;
        self
    }
}

impl Default for TuiInput {
    fn default() -> Self {
        Self::new()
    }
}

use crate::domain::styles_choice::StylesChoice;
