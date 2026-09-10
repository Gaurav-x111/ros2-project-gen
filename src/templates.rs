use crate::error::ProjectGenError;
use include_dir::{include_dir, Dir};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateMeta {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub required_features: Vec<String>,
    #[serde(default)]
    pub optional_features: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Template {
    pub meta: TemplateMeta,
    pub files: Vec<TemplateFile>,
}

#[derive(Debug, Clone)]
pub struct TemplateFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct TemplateEngine {
    templates: HashMap<String, Template>,
}

impl TemplateEngine {
    pub fn new() -> Result<Self, ProjectGenError> {
        let mut templates = HashMap::new();

        for dir in TEMPLATES_DIR.dirs() {
            let template_name = dir
                .path()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            if template_name.is_empty() {
                continue;
            }

            // Collect files
            let files = Self::collect_files(dir, "")?;

            // If template directory has no renderable files, skip it
            if files.is_empty() {
                continue;
            }

            // Parse metadata from template.toml if present
            let meta = Self::load_metadata(dir, &template_name);

            templates.insert(template_name.clone(), Template { meta, files });
        }

        Ok(Self { templates })
    }

    fn load_metadata(dir: &include_dir::Dir<'_>, template_name: &str) -> TemplateMeta {
        let default_meta = || TemplateMeta {
            name: template_name.to_string(),
            description: format!("{} template", template_name),
            version: "0.1.0".to_string(),
            required_features: vec![],
            optional_features: vec![],
        };

        // Walk the dir entries directly (get_file paths are root-relative)
        for entry in dir.entries() {
            if let include_dir::DirEntry::File(f) = entry {
                if f.path().file_name().map(|n| n == "template.toml").unwrap_or(false) {
                    match String::from_utf8(f.contents().to_vec()) {
                        Ok(content) => match toml::from_str::<TemplateMeta>(&content) {
                            Ok(meta) => return meta,
                            Err(e) => {
                                eprintln!(
                                    "warning: failed to parse {}/template.toml: {}",
                                    template_name, e
                                );
                                return default_meta();
                            }
                        },
                        Err(_) => return default_meta(),
                    }
                }
            }
        }
        default_meta()
    }

    fn collect_files(
        dir: &include_dir::Dir<'_>,
        prefix: &str,
    ) -> Result<Vec<TemplateFile>, ProjectGenError> {
        let mut files = Vec::new();

        for entry in dir.entries() {
            let name = entry.path().file_name().unwrap().to_string_lossy().to_string();

            // Skip template metadata file
            if name == "template.toml" {
                continue;
            }

            let relative_path = if prefix.is_empty() {
                name
            } else {
                format!("{}/{}", prefix, name)
            };

            match entry {
                include_dir::DirEntry::Dir(d) => {
                    files.extend(Self::collect_files(d, &relative_path)?);
                }
                include_dir::DirEntry::File(f) => {
                    let content = String::from_utf8(f.contents().to_vec()).map_err(|_| {
                        ProjectGenError::TemplateError(format!("Invalid UTF-8 in {}", relative_path))
                    })?;

                    // Strip .tera extension if present
                    let target_path = relative_path
                        .strip_suffix(".tera")
                        .unwrap_or(&relative_path)
                        .to_string();

                    files.push(TemplateFile {
                        path: target_path,
                        content,
                    });
                }
            }
        }

        Ok(files)
    }

    pub fn list_templates(&self) -> Vec<&TemplateMeta> {
        let mut metas: Vec<&TemplateMeta> = self.templates.values().map(|t| &t.meta).collect();
        metas.sort_by(|a, b| a.name.cmp(&b.name));
        metas
    }

    pub fn render_template(
        &self,
        template_name: &str,
        project_name: &str,
        output_dir: &Path,
    ) -> Result<(), ProjectGenError> {
        let template = self.templates.get(template_name).ok_or_else(|| {
            let names: Vec<String> = self.list_templates().iter().map(|m| m.name.clone()).collect();
            ProjectGenError::TemplateNotFound(template_name.to_string(), names.join(", "))
        })?;

        for file in &template.files {
            let rendered_path = Self::substitute_placeholders(&file.path, project_name);
            let rendered_content = Self::substitute_placeholders(&file.content, project_name);

            let output_path = output_dir.join(&rendered_path);
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&output_path, rendered_content)?;
        }

        Ok(())
    }

    fn substitute_placeholders(input: &str, project_name: &str) -> String {
        input
            .replace("{{project_name}}", project_name)
            .replace("{{ project_name }}", project_name)
            .replace("{{name}}", project_name)
            .replace("{{ name }}", project_name)
            .replace("{{PROJECT_NAME}}", &project_name.to_uppercase())
            .replace("{{ PROJECT_NAME }}", &project_name.to_uppercase())
            .replace("{{NAME}}", &project_name.to_uppercase())
            .replace("{{ NAME }}", &project_name.to_uppercase())
            .replace("{{project-name}}", &project_name.replace('_', "-"))
            .replace("{{ project-name }}", &project_name.replace('_', "-"))
            .replace("{{ros_distro}}", "jazzy")
            .replace("{{ ros_distro }}", "jazzy")
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new().expect("Failed to initialize template engine")
    }
}