use anyhow::Result;

use std::path::PathBuf;

pub fn initialize_working_folder(actual_path: PathBuf) -> Result<()> {

    println!("Actual path: {}", actual_path.display());

    Ok(())
}