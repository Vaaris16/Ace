use serde::Deserialize;
use std::path::PathBuf;

use crate::utilities::{
    edit_file::edit_file, errors::app_errors::AppErrors, mk_file::mk_file, run_cmd::run_cmd,
};

#[derive(Deserialize)]
pub struct InstructionPackage {
    steps: Vec<Steps>,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum Steps {
    #[serde(rename = "run_cmd")]
    RunCmd { command: String, args: String },

    #[serde(rename = "edit_file")]
    EditFile {
        needle: String,
        path: PathBuf,
        insert: String,
    },

    #[serde(rename = "mk_file")]
    MkFile { path: PathBuf, content: String },
}

impl Steps {
    pub fn execute_commands(&self) -> Result<(), AppErrors> {
        match self {
            Self::RunCmd { command, args } => run_cmd(command, args)?,
            Self::MkFile { path, content } => mk_file(path, content)?,
            Self::EditFile {
                needle,
                path,
                insert,
            } => edit_file(needle, path, insert)?,
        }

        Ok(())
    }
}

impl InstructionPackage {
    pub fn execute_steps(&self) -> Result<(), AppErrors> {
        for step in &self.steps {
            step.execute_commands()?;
        }

        Ok(())
    }
}
