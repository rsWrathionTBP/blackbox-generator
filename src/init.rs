use anyhow::Result;

use std::{path::PathBuf, fs, process};

pub fn initialize_working_folder(actual_path: &PathBuf) -> Result<()> {

    let path = env!("CARGO_MANIFEST_DIR");

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
    };

    println!("Done");



    Ok(())
}