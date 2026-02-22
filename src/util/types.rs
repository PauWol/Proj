use strum::IntoEnumIterator;
use strum_macros::{EnumIter, Display};

use crate::install::tools::{check_toolchain,install_toolchain};
use crate::util::exec::{is_tool_installed, run_checks_with_indicator,get_tool_version};

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
    /// Checks if the required tools for the project type are installed
    pub fn checks(&self) {
        
        let tools = check_toolchain(&self.project_type);
        
        if tools.iter().any(|(_, ok)| !*ok) {
            install_toolchain(&self.project_type);
        }
        
        
    }
}

