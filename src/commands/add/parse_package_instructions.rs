use std::fs;

use crate::commands::add::instructions::InstructionPackage;
use crate::utilities::errors::file_errors::FileErrors;

pub fn parse_package_instructions(
    package_name: &str,
    framework: &str,
) -> Result<Vec<InstructionPackage>, FileErrors> {
    let instructions_path = format!(
        "instruction/{}/{}_{}_instructions.toml",
        package_name, package_name, framework
    );

    let instructions_toml =
        fs::read_to_string(&instructions_path).map_err(|_| FileErrors::FailedRead)?;

    let instructions: Vec<InstructionPackage> =
        toml::from_str(&instructions_toml).map_err(|_| FileErrors::FailedParse)?;

    Ok(instructions)
}
