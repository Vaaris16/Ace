use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum FileErrors {
    FailedRead { file_path: PathBuf },
    FailedWrite { file_path: PathBuf },
    FailedParse { file_path: PathBuf },
}

impl Display for FileErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileErrors::FailedRead { file_path } => {
                write!(f, "Failed to read file: {:?}", file_path)
            }
            FileErrors::FailedWrite { file_path } => {
                write!(f, "Failed to create file: {:?}", file_path)
            }
            FileErrors::FailedParse { file_path } => {
                write!(f, "Failed to parse file: {:?}", file_path)
            }
        }
    }
}
