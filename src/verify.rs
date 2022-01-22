use anyhow::Result;

use crate::utils;
use std::{path::PathBuf, fs, process};


pub fn verify_assets(actual_path: PathBuf, assets_name: String) -> Result<()>{
    let mut assets_path = actual_path.clone();
    assets_path.push(&assets_name);

    let mut possible_error = false;
    let mut errors_array: Vec<String> = Vec::new();

    if !assets_path.exists() {
        println!("ERROR! {} folder does not exists in {}!", assets_name, actual_path.display());
        process::exit(1);
    }
    if !assets_path.is_dir() {
        println!("ERROR! {} is not a folder!", assets_name);
        process::exit(1);
    }

    println!("Verifying {} folder, all the possibles errors will be notified!", assets_name);

    let folders = fs::read_dir(&assets_path)?;

    let (images_shapes, images_with_shape) = utils::get_assets_files(folders)?;

    if images_shapes.len() <= 0 {
        println!("No files founded at {} folder, try using a non-empty assets folder", assets_name);
        return Ok(())
    }
    let (correct_size, _) = images_shapes.first().unwrap();
    if images_shapes.len() > 1 {
        possible_error = true;
        let mut err = format!("\tMultiples images sizes ({}) encountered:\n", images_shapes.len());
        for (key, count) in &images_shapes{
            err = format!("{}\t\t{} images have a size of ({},{})\n", err, count, key.width, key.height);
        }
        err = format!("{}\tAssuming correct size as ({},{})", err, correct_size.width, correct_size.height);
        errors_array.push(err);
    }

    for (key, _) in &images_shapes{
        for (parent_name, file_name) in &images_with_shape[&key]{
            if correct_size != key {
                let err = format!(
                    "\tYour file {} inside {} folder has a different size (Expected ({},{}), Found ({},{}))!",
                    file_name.to_string_lossy(), 
                    parent_name.to_string_lossy(),
                    correct_size.width, correct_size.height,
                    key.width, key.height
                );
                errors_array.push(err);
                possible_error = true;
            }
            
            let a = file_name.to_string_lossy().to_string();
            let a: Vec<&str> = a.trim().split('-').collect();
            if a.len() <= 1 {
                let err = format!(
                    "\tYour file {} inside {} folder is not correctly formatted (Missing '-')!",
                    file_name.to_string_lossy(), 
                    parent_name.to_string_lossy()
                );
                errors_array.push(err);
                possible_error = true;
                continue;
            }
            else if a.len() > 2{
                let err = format!(
                    "\tYour file {} inside {} folder is not correctly formatted (Have more than one '-')!",
                    file_name.to_string_lossy(), 
                    parent_name.to_string_lossy()
                );
                errors_array.push(err);
                possible_error = true;
                continue;
            }

            
            let number = match a[0].parse::<usize>(){
                Ok(number) => {number},
                Err(_) => {
                    let err = format!(
                        "\tYour file {} inside {} folder is not correctly formatted (Missing number before '-')!", 
                        file_name.to_string_lossy(), 
                        parent_name.to_string_lossy()
                    );
                    errors_array.push(err);
                    possible_error = true;
                    continue;
                },
            };

            if number <= 0 {
                let err = format!(
                    "\tYour file {} inside {} folder is not correctly formatted (Number is 0 or less than 0)!", 
                    file_name.to_string_lossy(), 
                    parent_name.to_string_lossy()
                );
                errors_array.push(err);
                possible_error = true;
            }
        }
    }
    if possible_error{
        println!("We encountered some incorrect formatted files, check the list below and the fix!");
        for err in errors_array{
            println!("{}",err);
        }
        return Ok(())
    }

    println!("Your files are correctly formated, you are ready to go!");
    Ok(())
}

