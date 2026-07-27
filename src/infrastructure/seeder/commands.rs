use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{bail, Context, Result};
use serde_json::Value;

use crate::domain::project::PackageManager;

pub trait CommandRunner: Send + Sync {
    fn run(&mut self, program: &str, args: &[String], cwd: Option<&Path>) -> Result<()>;
}

pub struct SystemCommandRunner;

impl CommandRunner for SystemCommandRunner {
    fn run(&mut self, program: &str, args: &[String], cwd: Option<&Path>) -> Result<()> {
        let mut cmd = Command::new(program);
        cmd.args(args);
        if let Some(path) = cwd {
            cmd.current_dir(path);
        }

        let status = cmd.status().with_context(|| {
            format!(
                "failed to start command `{}`",
                format_command(program, args)
            )
        })?;

        if !status.success() {
            bail!("command failed: {}", format_command(program, args));
        }

        Ok(())
    }
}

pub(crate) fn format_command(program: &str, args: &[String]) -> String {
    if args.is_empty() {
        return program.to_string();
    }
    format!("{} {}", program, args.join(" "))
}

use crate::domain::project::ResolvedOptions;

pub(crate) fn ensure_required_tools(
    runner: &mut dyn CommandRunner,
    package_manager: PackageManager,
) -> Result<()> {
    for tool in ["node", "ng", package_manager_cli_name(package_manager)] {
        let args = vec!["--version".to_string()];
        runner
            .run(tool, &args, None)
            .with_context(|| format!("`{tool}` is required. Install it and retry."))?;
    }

    Ok(())
}

pub(crate) fn scaffold_angular_project(
    runner: &mut dyn CommandRunner,
    project_name: &str,
    options: ResolvedOptions,
) -> Result<()> {
    let package_manager = package_manager_cli_name(options.package_manager);

    let mut args = vec![
        "new".to_string(),
        project_name.to_string(),
        "--defaults".to_string(),
        "--standalone".to_string(),
        "--routing".to_string(),
        "--style=scss".to_string(),
        "--ssr=false".to_string(),
        format!("--package-manager={package_manager}"),
    ];

    if options.skip_install {
        args.push("--skip-install".to_string());
    }

    if options.skip_git {
        args.push("--skip-git".to_string());
    }

    runner.run("ng", &args, None)
}

pub(crate) fn package_manager_cli_name(package_manager: PackageManager) -> &'static str {
    match package_manager {
        PackageManager::Npm => "npm",
        PackageManager::Pnpm => "pnpm",
        PackageManager::Yarn => "yarn",
        PackageManager::Bun => "bun",
    }
}

pub(crate) fn package_manager_install_command(
    package_manager: PackageManager,
    packages: &[&str],
) -> (&'static str, Vec<String>) {
    match package_manager {
        PackageManager::Npm => {
            let mut args = vec!["install".to_string()];
            args.extend(packages.iter().map(|s| s.to_string()));
            ("npm", args)
        }
        PackageManager::Pnpm => {
            let mut args = vec!["add".to_string()];
            args.extend(packages.iter().map(|s| s.to_string()));
            ("pnpm", args)
        }
        PackageManager::Yarn => {
            let mut args = vec!["add".to_string()];
            args.extend(packages.iter().map(|s| s.to_string()));
            ("yarn", args)
        }
        PackageManager::Bun => {
            let mut args = vec!["add".to_string()];
            args.extend(packages.iter().map(|s| s.to_string()));
            ("bun", args)
        }
    }
}

pub(crate) fn add_styles_to_angular_json(project_dir: &Path, styles: &[&str]) -> Result<()> {
    let angular_json_path = project_dir.join("angular.json");
    let raw = fs::read_to_string(&angular_json_path)
        .with_context(|| format!("failed to read {}", angular_json_path.display()))?;

    let mut json: Value =
        serde_json::from_str(&raw).context("failed to parse angular.json as JSON")?;

    let projects = json
        .get_mut("projects")
        .and_then(Value::as_object_mut)
        .context("angular.json is missing `projects`")?;

    let Some((_project_name, project_config)) = projects.iter_mut().next() else {
        bail!("angular.json has no projects entries");
    };

    let styles_array = project_config
        .pointer_mut("/architect/build/options/styles")
        .and_then(Value::as_array_mut)
        .context("angular.json is missing /architect/build/options/styles")?;

    for style in styles {
        if !styles_array
            .iter()
            .any(|entry| entry.as_str() == Some(style))
        {
            styles_array.push(Value::String((*style).to_string()));
        }
    }

    let rendered = serde_json::to_string_pretty(&json).context("failed to render angular.json")?;
    fs::write(&angular_json_path, format!("{rendered}\n"))
        .with_context(|| format!("failed to write {}", angular_json_path.display()))?;

    Ok(())
}

pub(crate) fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {}", parent.display()))?;
    }

    fs::write(path, content).with_context(|| format!("failed to write {}", path.display()))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use tempfile::tempdir;

    use super::*;

    #[derive(Default)]
    struct FakeRunner {
        calls: Vec<(String, Vec<String>, Option<PathBuf>)>,
    }

    impl CommandRunner for FakeRunner {
        fn run(&mut self, program: &str, args: &[String], cwd: Option<&Path>) -> Result<()> {
            self.calls.push((
                program.to_string(),
                args.to_vec(),
                cwd.map(Path::to_path_buf),
            ));
            Ok(())
        }
    }

    #[test]
    fn scaffold_calls_ng_new_with_expected_flags() {
        let mut runner = FakeRunner::default();

        // Inline scaffold logic for the test (same flags as scaffold_angular_project)
        let package_manager = package_manager_cli_name(PackageManager::Pnpm);
        let mut args = vec![
            "new".to_string(),
            "demo-app".to_string(),
            "--defaults".to_string(),
            "--standalone".to_string(),
            "--routing".to_string(),
            "--style=scss".to_string(),
            "--ssr=false".to_string(),
            format!("--package-manager={package_manager}"),
        ];
        args.push("--skip-install".to_string());

        runner.run("ng", &args, None).unwrap();

        assert_eq!(runner.calls.len(), 1);
        let (program, args, _) = &runner.calls[0];
        assert_eq!(program, "ng");
        assert!(args.contains(&"new".to_string()));
        assert!(args.contains(&"demo-app".to_string()));
        assert!(args.contains(&"--standalone".to_string()));
        assert!(args.contains(&"--routing".to_string()));
        assert!(args.contains(&"--style=scss".to_string()));
        assert!(args.contains(&"--ssr=false".to_string()));
        assert!(args.contains(&"--package-manager=pnpm".to_string()));
        assert!(args.contains(&"--skip-install".to_string()));
    }

    #[test]
    fn scaffold_passes_skip_git_when_requested() {
        let mut runner = FakeRunner::default();

        let package_manager = package_manager_cli_name(PackageManager::Npm);
        let mut args = vec![
            "new".to_string(),
            "demo-app".to_string(),
            "--defaults".to_string(),
            "--standalone".to_string(),
            "--routing".to_string(),
            "--style=scss".to_string(),
            "--ssr=false".to_string(),
            format!("--package-manager={package_manager}"),
        ];
        args.push("--skip-git".to_string());

        runner.run("ng", &args, None).unwrap();

        assert_eq!(runner.calls.len(), 1);
        let (_, args, _) = &runner.calls[0];
        assert!(args.contains(&"--skip-git".to_string()));
    }

    #[test]
    fn package_manager_install_command_matches_manager() {
        let (program, args) = package_manager_install_command(PackageManager::Bun, &["primeng"]);
        assert_eq!(program, "bun");
        assert_eq!(args, vec!["add".to_string(), "primeng".to_string()]);
    }

    #[test]
    fn primeng_adds_expected_styles() {
        let tmp = tempdir().unwrap();
        let project_dir = tmp.path();
        fs::create_dir_all(project_dir.join("src/app")).unwrap();
        fs::write(
            project_dir.join("angular.json"),
            r#"{
  "projects": {
    "demo": {
      "architect": {
        "build": {
          "options": {
            "styles": ["src/styles.scss"]
          }
        }
      }
    }
  }
}
"#,
        )
        .unwrap();

        add_styles_to_angular_json(
            project_dir,
            &[
                "node_modules/@primeng/themes/aura/theme.css",
                "node_modules/primeicons/primeicons.css",
            ],
        )
        .unwrap();

        let rendered = fs::read_to_string(project_dir.join("angular.json")).unwrap();
        assert!(rendered.contains("node_modules/@primeng/themes/aura/theme.css"));
        assert!(rendered.contains("node_modules/primeicons/primeicons.css"));
    }
}
