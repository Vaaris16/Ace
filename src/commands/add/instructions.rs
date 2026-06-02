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
#[serde(tag = "action", rename_all = "snake_case")]
pub enum Steps {
    RunCmd {
        commands: String,
        args: String,
    },
    EditFile {
        needle: String,
        path: PathBuf,
        insert: String,
    },
    MkFile {
        path: PathBuf,
        contents: String,
    },
}

impl Steps {
    pub fn execute_commands(&self) -> Result<(), AppErrors> {
        match self {
            Self::RunCmd { commands, args } => run_cmd(commands, args)?,
            Self::MkFile { path, contents } => mk_file(path, contents)?,
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
