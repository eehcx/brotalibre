use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::domain::project::PackageManager;
use crate::domain::styles_choice::StylesChoice;
use crate::infrastructure::seeder::commands::write_file;
use crate::infrastructure::seeder::commands::package_manager_install_command;
use crate::infrastructure::seeder::templates::TemplateLoader;
use crate::infrastructure::seeder::CommandRunner;

pub(crate) fn apply_styles(
    runner: &mut dyn CommandRunner,
    template_base: &Path,
    project_dir: &Path,
    styles: StylesChoice,
    package_manager: PackageManager,
) -> Result<()> {
    match styles {
        StylesChoice::None => Ok(()),
        StylesChoice::TailwindCSS => {
            let (program, install_args) = package_manager_install_command(
                package_manager,
                &["tailwindcss", "@tailwindcss/postcss"],
            );
            runner.run(program, &install_args, Some(project_dir))?;

            write_file(
                &project_dir.join("postcss.config.js"),
                r#"module.exports = {
  plugins: {
    '@tailwindcss/postcss': {},
  },
};
"#,
            )?;

            let loader = TemplateLoader::new(template_base)?;
            let styles_scss = project_dir.join("src/styles.scss");
            fs::write(&styles_scss, loader.render("styles.scss.j2", ())?)
                .with_context(|| format!("failed to write {}", styles_scss.display()))?;

            Ok(())
        }
    }
}
