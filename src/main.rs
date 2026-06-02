use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::Parser;

use crate::utilities::edit_file::edit_file;

mod commands;
mod utilities;

#[derive(Parser, Debug)]
struct Cli {
    name: String,
}

fn main() {
    let cli = Cli::parse();

    println!("name: {}", cli.name);
}
