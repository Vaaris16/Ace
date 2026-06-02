use core::fmt;

#[derive(Debug)]
pub enum CmdErrors {
    NotFound,
    PermissionDenied,
    InvalidInput,
    Interrupted,
    Other,
}

impl fmt::Display for CmdErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CmdErrors::NotFound => write!(f, "Command not found"),
            CmdErrors::PermissionDenied => write!(f, "Permission denied"),
            CmdErrors::InvalidInput => write!(f, "Invalid input"),
            CmdErrors::Interrupted => write!(f, "Execution interrupted"),
            CmdErrors::Other => write!(f, "unknown error"),
        }
    }
}
