use std::path::Path;

use anyhow::Result;

use crate::application::ports::ProgressReporter;
use crate::application::ports::Seeder;
use crate::domain::project::GenerateFeatureRequest;

pub struct GenerateFeatureUseCase<'a> {
    seeder: &'a dyn Seeder,
    reporter: &'a dyn ProgressReporter,
}

impl<'a> GenerateFeatureUseCase<'a> {
    pub fn new(seeder: &'a dyn Seeder, reporter: &'a dyn ProgressReporter) -> Self {
        Self { seeder, reporter }
    }

    pub fn execute(&self, request: GenerateFeatureRequest) -> Result<()> {
        let project_dir = Path::new(&request.project_dir);

        self.reporter
            .stage_start("generate", &format!("generating {} feature", request.name));

        self.seeder.apply_feature_template(
            project_dir,
            request.architecture,
            &request.name,
            &request.prefix,
            &request.fields,
        )?;

        self.reporter
            .stage_ok("generate", &format!("feature {} created", request.name));

        Ok(())
    }
}
