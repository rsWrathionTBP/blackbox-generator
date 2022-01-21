use anyhow::Result;
use imagesize::{ImageSize, size};
use std::{collections::HashMap};

use crate::utils;
use std::{path::PathBuf, fs, process};


pub fn verify_assets(actual_path: PathBuf, assets_name: String) -> Result<()>{
    let mut assets_path = actual_path.clone();
    assets_path.push(&assets_name);
    println!("{}", assets_path.display());

    if !assets_path.exists(){
        println!("\tERROR! {} folder does not exists in {}", assets_name, actual_path.display());
        process::exit(1);
    }
    if !assets_path.is_dir(){
        println!("\tERROR! {} is not a folder", assets_name);
        process::exit(1);
    }

    let folders = fs::read_dir(assets_path)?;
    let mut images_shapes_keys: Vec<ImageSize> = Vec::new();
    let mut images_shapes: HashMap<ImageSize, usize> = HashMap::new();
    let mut images_with_shape: HashMap<ImageSize, Vec<PathBuf>> = HashMap::new();
    for folder in folders{
        let folder = folder?;
        let folder_name = folder.file_name();
        if !folder.path().is_dir(){
            println!("\tERROR! {} is not a folder", folder.path().display());
            continue;
            //process::exit(1);
        }
        let files = fs::read_dir(folder.path())?;

        for file in files{
            let file = file?;
            let file_name = file.file_name();
            if !utils::is_image(&file){
                println!("TF is this");
                continue;
            }

            let size = match size(file.path()){
                Ok(size) => {size},
                Err(_) => {
                    println!("\tERROR! Unable to get shape of {} inside {} folder, with error: ", file_name.to_string_lossy(), folder_name.to_string_lossy());
                    continue;
                }
            };
            
            if !images_shapes.contains_key(&size){
                images_shapes.insert(size, 1);
                images_with_shape.insert(size, vec![file.path()]);
                images_shapes_keys.push(size);
            }
            else{
                images_shapes.insert(size, 1+ images_shapes[&size]);
                let mut new_vec = images_with_shape[&size].clone();
                new_vec.push(file.path());
                images_with_shape.insert(size, new_vec);
            }
            //println!("\t{} ({},{})",file_name.to_string_lossy(), size.width, size.height);
        }
    }
    println!("{:?}", images_shapes);
    println!( "{:?}", images_shapes_keys);
    println!("\n{:#?}", images_with_shape);
    

    Ok(())
}

