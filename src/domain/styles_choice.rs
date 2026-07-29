#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StylesChoice {
    #[default]
    Css,
    Scss,
    Sass,
    Less,
    TailwindCSS,
}

impl StylesChoice {
    pub fn angular_cli_value(self) -> &'static str {
        match self {
            StylesChoice::Css => "css",
            StylesChoice::Scss => "scss",
            StylesChoice::Sass => "sass",
            StylesChoice::Less => "less",
            StylesChoice::TailwindCSS => "tailwind",
        }
    }

    pub fn file_extension(self) -> &'static str {
        match self {
            StylesChoice::TailwindCSS => "css",
            _ => self.angular_cli_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_styles_to_angular_cli_values_and_component_extensions() {
        assert_eq!(StylesChoice::Css.angular_cli_value(), "css");
        assert_eq!(StylesChoice::Scss.angular_cli_value(), "scss");
        assert_eq!(StylesChoice::Sass.angular_cli_value(), "sass");
        assert_eq!(StylesChoice::Less.angular_cli_value(), "less");
        assert_eq!(StylesChoice::TailwindCSS.angular_cli_value(), "tailwind");
        assert_eq!(StylesChoice::TailwindCSS.file_extension(), "css");
    }
}
