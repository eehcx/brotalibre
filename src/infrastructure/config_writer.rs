use std::path::Path;

use anyhow::Result;

use crate::application::ports::{ConfigReader, ConfigWriter};
use crate::domain::project_config::{BROTA_CONFIG_FILENAME, ProjectConfig};

pub struct SystemConfigWriter;

impl ConfigWriter for SystemConfigWriter {
    fn write_config(&self, project_dir: &Path, config: &ProjectConfig) -> Result<()> {
        let path = project_dir.join(BROTA_CONFIG_FILENAME);
        let yaml = serde_yaml_ng::to_string(config)?;
        std::fs::write(&path, yaml)?;
        Ok(())
    }
}

pub struct SystemConfigReader;

impl ConfigReader for SystemConfigReader {
    fn read_config(&self, project_dir: &Path) -> Result<ProjectConfig> {
        let path = project_dir.join(BROTA_CONFIG_FILENAME);
        let yaml = std::fs::read_to_string(&path).map_err(|error| {
            anyhow::anyhow!(
                "could not read {}: {error}; run `brota new` in this directory first",
                path.display()
            )
        })?;

        serde_yaml_ng::from_str(&yaml)
            .map_err(|error| anyhow::anyhow!("could not parse {}: {error}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn reads_project_config_from_brota_yaml() {
        let directory = tempdir().unwrap();
        fs::write(
            directory.path().join(BROTA_CONFIG_FILENAME),
            "schemaVersion: '1'\nproject:\n  name: demo\nprofile: angular-admin\ntarget:\n  framework: angular\n  architecture: feature-clean\n  packageManager: npm\nui:\n  library: material\n  styleEngine: scss\n",
        )
        .unwrap();

        let config = SystemConfigReader.read_config(directory.path()).unwrap();

        assert_eq!(config.project.name, "demo");
        assert_eq!(config.target.framework, "angular");
    }

    #[test]
    fn rejects_malformed_brota_yaml() {
        let directory = tempdir().unwrap();
        fs::write(
            directory.path().join(BROTA_CONFIG_FILENAME),
            "schemaVersion: [not valid",
        )
        .unwrap();

        let error = SystemConfigReader
            .read_config(directory.path())
            .unwrap_err()
            .to_string();

        assert!(error.contains("could not parse"));
    }
}
