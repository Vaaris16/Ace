use std::{io::ErrorKind, process::Command};

use crate::utilities::errors::{app_errors::AppErrors, cmd_errors::CmdErrors};

pub fn run_cmd(commands: &str, args: &str) -> Result<(), AppErrors> {
    Command::new(commands)
        .args(args.split_whitespace())
        .output()
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => AppErrors::CmdErrors(CmdErrors::NotFound),
            ErrorKind::PermissionDenied => AppErrors::CmdErrors(CmdErrors::PermissionDenied),
            ErrorKind::InvalidInput => AppErrors::CmdErrors(CmdErrors::InvalidInput),
            ErrorKind::Interrupted => AppErrors::CmdErrors(CmdErrors::Interrupted),
            _ => AppErrors::CmdErrors(CmdErrors::Other),
        })?;

    Ok(())
}
