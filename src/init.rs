use anyhow::Result;

use std::{
    path::PathBuf,
    fs,
    process, env
};

use fs_extra;


/// Initialize a new working folder to generate Solana collections
pub fn initialize_solana_working_folder(actual_path: PathBuf, name: String) -> Result<()> {
    println!("Initializing new solana workspace folder at: {} with name: {}-solana-workspace",actual_path.display(), name);

    let mut template_path = PathBuf::new();
    template_path.push(env!("CARGO_MANIFEST_DIR"));
    template_path.push("templates");

    let mut working_path = actual_path.clone();
    working_path.push(name.clone() + "-solana-workspace");
    
    println!("\tCreating {}-solana-workspace folder at: {}", name, actual_path.display());
    if working_path.exists(){
        println!("\tERROR! {}-solana-workspace already exists in this path, try using a another name!", name);
        process::exit(1);
    }
    match fs::create_dir(&working_path){
        Ok(_) => {println!("\tSUCCESS! {}-solana-workspace folder created", name)},
        Err(e) => {
            println!("\tERROR! Unable to create {}-solana-workspace folder, with error: {}", name, e);
            process::exit(1);
        }
    };

    println!("\tSetting new working path to: {}", working_path.display());
    match env::set_current_dir(&working_path){
        Ok(_) => {println!("\tSUCCESS! Working path setted")},
        Err(e) => {
            println!("\tERROR! Unable to set new working path, with error: {}", e);
            process::exit(1);
        }
    };

    let mut config_path = template_path.clone();
    config_path.push("config-solana.json");

    println!("\tCreating config.json file at: {}", working_path.display());
    match fs::copy(config_path, "config.json"){
        Ok(_) => {println!("\tSUCCESS! config.json file created")},
        Err(e) => {
            println!("\tERROR! Unable to create config.json file, with error {}", e);
            process::exit(1);
        }
    }

    println!("\tCreating assets folder at: {}", actual_path.display());
    match fs::create_dir(&"assets"){
        Ok(_) => {println!("\tSUCCESS! assets folder created")},
        Err(e) => {
            println!("\tERROR! Unable to create assets folder, with error: {}", e);
            process::exit(1);
        }
    };

    let mut example_config_path = template_path.clone();
    example_config_path.push("example-config-solana.json");

    println!("\tCreating example-config.json file at: {}", working_path.display());
    match fs::copy(example_config_path, "example-config.json"){
        Ok(_) => {println!("\tSUCCESS! example-config.json file created")},
        Err(e) => {
            println!("\tERROR! Unable to create example-config.json file, with error {}", e);
            process::exit(1);
        }
    }

    let mut assets_path = template_path.clone();
    assets_path.push("example-assets");
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.copy_inside = true;
    println!("\tCreating example-assets folder at: {}", working_path.display());
    match fs_extra::dir::copy(assets_path, "example-assets", &copy_options){
        Ok(_) => {println!("\tSUCCESS! example-assets folder created")},
        Err(e) => {
            println!("\tERROR! Unable to create the example-assets folder, with error {}", e);
            process::exit(1);
        }
    }
    println!("{}-solana-workspace initialized succesfully!", name);
    Ok(())
}

/// Initialize a new working folder to generate Ethereum collections
pub fn initialize_ethereum_working_folder(actual_path: PathBuf, name: String) -> Result<()> {
    println!("Initializing new Ethereum working folder at {} with name {}",actual_path.display(), name);
    /*let path = env!("CARGO_MANIFEST_DIR");

    let mut working_path = actual_path.clone();
    working_path.push("blackbox-workspace");

    println!("Actual path: {}, {}, {}", actual_path.display(), working_path.display() , path);

    if working_path.exists(){
        println!("ERROR! working folder already exists in this path!");
        process::exit(1);
    }

    match fs::create_dir(&working_path){
        Ok(_) => {
            println!("Your working folder have the following path: {:}", working_path.display())
        },
        Err(e) => {
            println!("ERROR UNABLE TO CREATE THE FOLDER -> {:}", e);
            process::exit(1);
        }
    };*/

    println!("SUCCESS! {}-ethereum-workspace initialized :D", name);



    Ok(())
}