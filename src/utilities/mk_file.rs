use std::{fs, path::PathBuf};

use crate::utilities::errors::{app_errors::AppErrors, file_errors::FileErrors};

pub fn mk_file(path: &PathBuf, contents: &str) -> Result<(), AppErrors> {
    if !path.exists() {
        fs::write(&path, contents).map_err(|_| AppErrors::FileErrors(FileErrors::FailedWrite))?;
    }
    println!("file exists");

    Ok(())
}
