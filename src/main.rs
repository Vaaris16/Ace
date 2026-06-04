use clap::{Parser, Subcommand};

use crate::{
    commands::add::install_package::install_packages, frameworks::frameworks::Frameworks,
    utilities::errors::app_errors::AppErrors,
};

mod commands;
mod frameworks;
mod utilities;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        package: String,

        #[arg(short, long, required = true)]
        framework: Frameworks,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Add { package, framework } => {
            if let Err(e) = install_packages(&package, framework) {
                println!("{:?}", e);
            }
        }
    }
}
