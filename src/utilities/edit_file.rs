use std::{fs, path::PathBuf};

use crate::utilities::errors::{app_errors::AppErrors, file_errors::FileErrors};

pub fn edit_file(needle: &str, path: &PathBuf, insert: &str) -> Result<(), AppErrors> {
    let content =
        fs::read_to_string(&path).map_err(|_| AppErrors::FileErrors(FileErrors::FailedRead))?;

    let new_content = match needle {
        "^" => format!("{}\n{}", insert, content),
        "&" => format!("{}\n{}", content, insert),
        _ => {
            if let Some(pos) = content.find(needle) {
                let index = pos + needle.len();
                format!("{}{}{}", &content[..index], &insert, &content[index..],)
            } else {
                content
            }
        }
    };

    fs::write(&path, new_content).map_err(|_| AppErrors::FileErrors(FileErrors::FailedWrite))?;
    Ok(())
}
