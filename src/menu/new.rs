use crate::util::types::{ProjectType, TemplateType};
use inquire::{Text, Select, validator::Validation};

pub fn new_project_menu() -> (String, ProjectType,TemplateType) {
    let name = Text::new("Enter project name:")
            .with_validator(|input: &str| {
                if input.trim().is_empty() {
                    Ok(Validation::Invalid("Name cannot be empty".into()))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .prompt()
            .expect("Failed to read name");

    let project_type = Select::new("Select project type:", ProjectType::variants()
    ).prompt().expect("Failed to read project type");

    let template_type = Select::new("Select template type:", TemplateType::variants()
    ).prompt().expect("Failed to read template type");

    (name, project_type, template_type)
    
}