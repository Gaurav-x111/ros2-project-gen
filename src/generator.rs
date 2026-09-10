use crate::error::ProjectGenError;
use crate::templates::TemplateEngine;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct ProjectGenerator {
    template_engine: TemplateEngine,
}

impl ProjectGenerator {
    pub fn new() -> Result<Self, ProjectGenError> {
        Ok(Self {
            template_engine: TemplateEngine::new()?,
        })
    }

    pub fn validate_project_name(&self, name: &str) -> Result<(), ProjectGenError> {
        if name.is_empty() {
            return Err(ProjectGenError::EmptyName);
        }
        if !regex::Regex::new(r"^[a-zA-Z0-9_-]+$")
            .unwrap()
            .is_match(name)
        {
            return Err(ProjectGenError::InvalidName(name.to_string()));
        }
        if name.contains("..") || name.contains('/') || name.contains('\\') {
            return Err(ProjectGenError::PathTraversal(name.to_string()));
        }
        Ok(())
    }

    pub fn validate_output_dir(
        &self,
        path: &Path,
        project_name: &str,
    ) -> Result<(), ProjectGenError> {
        let project_path = path.join(project_name);
        if project_path.exists() {
            let entries: Vec<_> = WalkDir::new(&project_path)
                .min_depth(1)
                .max_depth(1)
                .into_iter()
                .collect();
            if !entries.is_empty() {
                return Err(ProjectGenError::DirectoryNotEmpty(
                    project_path.display().to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn generate(
        &self,
        project_name: &str,
        template: &str,
        output_dir: &Path,
    ) -> Result<PathBuf, ProjectGenError> {
        self.validate_project_name(project_name)?;
        self.validate_output_dir(output_dir, project_name)?;

        let project_path = output_dir.join(project_name);
        std::fs::create_dir_all(&project_path)?;

        self.template_engine
            .render_template(template, project_name, &project_path)?;

        Ok(project_path)
    }

    pub fn list_templates(&self) -> Vec<&crate::templates::TemplateMeta> {
        self.template_engine.list_templates()
    }
}

impl Default for ProjectGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create project generator")
    }
}
