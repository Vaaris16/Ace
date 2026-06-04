use crate::utilities::errors::{cmd_errors::CmdErrors, file_errors::FileErrors};

#[derive(Debug)]
pub enum AppErrors {
    FileErrors(FileErrors),
    CmdErrors(CmdErrors),
}
