mod cli;
mod menu;
mod util;
use clap::Parser;

use cli::root::{Cli, Commands};
use menu::new::new_project_menu;
use util::types::{ProjectType, TemplateType,Project};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, template } => {

            if name == None {
                let (name, project_type, template_type) = new_project_menu();
                let project = Project {
                    name,
                    project_type,
                    template_type,
                };
                
                project.checks();
                return;
            }
            println!("Creating project...");
            println!("Name: {:?}", name);
            println!("Template: {:?}", template);


        }

        Commands::List => {
            println!("Listing templates...");
        }

        Commands::AddTemplate { path } => {
            println!("Adding template from: {}", path);
        }
    }
}