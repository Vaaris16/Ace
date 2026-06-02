use crate::utilities::errors::{cmd_errors::CmdErrors, file_errors::FileErrors};

pub enum AppErrors {
    FileErrors(FileErrors),
    CmdErrors(CmdErrors),
}
