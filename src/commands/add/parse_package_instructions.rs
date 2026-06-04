use crate::commands::add::instructions::InstructionPackage;
use crate::frameworks::frameworks::Frameworks;
use crate::utilities::errors::app_errors::AppErrors;
use crate::utilities::errors::file_errors::FileErrors;
use std::path::PathBuf;

pub fn parse_package_instructions(
    package_name: &str,
    framework: Frameworks,
) -> Result<InstructionPackage, AppErrors> {
    let instructions_path: &str = match (package_name, framework) {
        ("tailwindcss", Frameworks::React) => {
            include_str!("./instructions/tailwindcss/tailwindcss_react_instructions.toml")
        }
        _ => "not found",
    };

    let instructions: InstructionPackage = toml::from_str(&instructions_path).map_err(|_| {
        AppErrors::FileErrors(FileErrors::FailedParse {
            file_path: PathBuf::from(&instructions_path),
        })
    })?;

    Ok(instructions)
}
