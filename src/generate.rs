use std::path::PathBuf;

use anyhow::Result;

pub fn generate_sample(actual_path: PathBuf, assets_folder: String, config_json: String) -> Result<()>{
    println!("Generating samples -> {} {} {}", actual_path.display(), assets_folder, config_json);
    Ok(())    
}

pub fn generate_nfts(actual_path: PathBuf, sample_json: String) -> Result<()>{
    println!("Generating nfts -> {} {}", actual_path.display(), sample_json);
    Ok(())    
}

pub fn generate_all(actual_path: PathBuf, assets_folder: String, config_json: String) -> Result<()>{
    generate_sample(actual_path.clone(), assets_folder.clone(), config_json.clone())?;
    generate_nfts(actual_path.clone(), "samples.json".to_string())?;
    Ok(())

}