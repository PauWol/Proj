use strum::IntoEnumIterator;
use strum_macros::{EnumIter, Display};

use crate::util::checks::is_installed_with_ui;

#[derive(Debug, Clone, EnumIter, Display)]
pub enum ProjectType {
    Rust,
    Python,
    Node,
    Go,
    Java,
}

impl ProjectType {
    pub fn variants() -> Vec<ProjectType> {
        ProjectType::iter().collect()
    }

    pub fn lang_command(&self) -> &str {
        match self {
            ProjectType::Rust => "rustc",
            ProjectType::Python => "python",
            ProjectType::Node => "node",
            ProjectType::Go => "go",
            ProjectType::Java => "java",
        }
    }

    pub fn lang_version_arg(&self) -> &str {
        match self {
            ProjectType::Rust => "--version",
            ProjectType::Python => "--version",
            ProjectType::Node => "--version",
            ProjectType::Go => "version",
            ProjectType::Java => "-version",
        }
    }
    
}

#[derive(Debug, Clone, EnumIter, Display)]
pub enum TemplateType {
    Basic,
    Web,
    CLI,
    Api,
    Tool,
    Library
}

impl TemplateType {
    pub fn variants() -> Vec<TemplateType> {
        TemplateType::iter().collect()
    }
}


pub struct Project {
    pub name: String,
    pub project_type: ProjectType,
    pub template_type: TemplateType,
}

impl Project {
    pub fn new(name: String, project_type: ProjectType, template_type: TemplateType) -> Self {
        Project {
            name,
            project_type,
            template_type,
        }
    }

    pub fn checks(&self) -> bool {
        is_installed_with_ui(&self.project_type)
    }
}