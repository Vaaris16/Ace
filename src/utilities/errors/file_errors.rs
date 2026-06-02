use std::fmt::Display;

#[derive(Debug)]
pub enum FileErrors {
    FileExists,
    NotFound,
    PermissionDenied,
    FailedRead,
    FailedWrite,
    FailedParse,
}

impl Display for FileErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileErrors::FileExists => write!(f, "File already exists"),
            FileErrors::NotFound => write!(f, "File not found"),
            FileErrors::PermissionDenied => write!(f, "Permission denied"),
            FileErrors::FailedRead => write!(f, "Failed to read file"),
            FileErrors::FailedWrite => write!(f, "Failed to create file"),
            FileErrors::FailedParse => write!(f, "Failed to parse file"),
        }
    }
}
