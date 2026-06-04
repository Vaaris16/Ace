use crate::{
    commands::add::parse_package_instructions::parse_package_instructions,
    frameworks::frameworks::Frameworks, utilities::errors::app_errors::AppErrors,
};

pub fn install_packages(package_name: &str, framework: Frameworks) -> Result<(), AppErrors> {
    let instructions_command = parse_package_instructions(package_name, framework)?;

    instructions_command.execute_steps()?;

    Ok(())
}
