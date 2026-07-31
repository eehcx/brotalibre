use std::path::Path;

use anyhow::Result;

use crate::application::ports::ConfigWriter;
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
