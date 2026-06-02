use crate::commands::add::parse_package_instructions::parse_package_instructions;

pub fn install_packages(package_name: &str, framework: &str) {
    let instructions_command = parse_package_instructions(package_name, framework).unwrap();

    for instruction in instructions_command {
        instruction.execute_steps();
    }
}
