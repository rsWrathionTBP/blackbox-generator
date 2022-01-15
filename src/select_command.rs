use anyhow::Result;

use std::path::PathBuf;

use crate::init::initialize_working_folder;

pub fn command_init(actual_path: PathBuf) -> Result<()> {
    initialize_working_folder(actual_path)?;
    Ok(())
}

pub fn command_verify() -> Result<()>{
    todo!();
}

pub fn command_config() -> Result<()>{
    todo!();
}

pub fn command_edit() -> Result<()>{
    todo!();
}

pub fn command_generate() -> Result<()>{
    todo!();
}

pub fn command_rarity() -> Result<()>{
    todo!();
}
