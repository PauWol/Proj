use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "proj")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New {
        name: Option<String>,

        #[arg(short, long)]
        template: Option<String>,
    },
    List,
    AddTemplate {
        path: String,
    },
}