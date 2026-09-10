use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectGenError {
    #[error("Invalid project name: {0}. Project names must contain only alphanumeric characters, hyphens, and underscores.")]
    InvalidName(String),

    #[error("Project directory already exists and is not empty: {0}")]
    DirectoryNotEmpty(String),

    #[error("Path traversal detected in project name: {0}")]
    PathTraversal(String),

    #[error("Template not found: {0}. Available templates: {1}")]
    TemplateNotFound(String, String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Template rendering error: {0}")]
    TemplateError(String),

    #[error("Project name cannot be empty")]
    EmptyName,
}
