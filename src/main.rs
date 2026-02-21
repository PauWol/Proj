mod cli;
mod menu;
mod util;
mod install;
use clap::Parser;

use cli::root::{Cli, Commands};
use menu::new::new_project_menu;
use util::types::{Project};

fn main() {
    let cli = Cli::parse();

    match cli.command {

        // User invoked the "new" command
        Commands::New { name, template } => {


            // Invoke Menu-Flow if no name is provided
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

        }

        // User invoked the "list" command
        Commands::List => {
            println!("Listing templates...");
        }

        // User invoked the "add-template" command
        Commands::AddTemplate { path } => {
            println!("Adding template from: {}", path);
        }
    }
}