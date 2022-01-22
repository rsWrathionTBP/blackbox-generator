use std::{fs::{self, ReadDir}, collections::HashMap, ffi::OsString};
use anyhow::Result;
use imagesize::{ImageSize, size};
use indexmap::IndexMap;

/// Check which file inside each layer is an image (png, jpg, gif)
pub fn is_image(file: &fs::DirEntry) -> bool{
    if !file.path().is_file(){
        return false
    }
    else if let Some(extension) = file.path().extension() {
        if extension == "png" || extension == "jpg" || extension == "gif"{
            //println!("{}", extension.to_string_lossy());
            return true;
        }
        else if extension == "jpeg"{
            println!()
        }
    }
    return false;
}

pub fn get_assets_files(folders: ReadDir) -> Result<(IndexMap<ImageSize, usize>, HashMap<ImageSize, Vec<(OsString, OsString)>>)>{
    let mut images_shapes:IndexMap<ImageSize, usize> = IndexMap::new();
    let mut images_with_shape: HashMap<ImageSize, Vec<(OsString, OsString)>> = HashMap::new();

    for folder in folders{
        let folder = folder?;
        
        let folder_name = folder.file_name();
        if !folder.path().is_dir(){
            continue;
        }

        let files = fs::read_dir(folder.path())?;

        for file in files{
            let file = file?;
            let file_name = file.file_name();
            if !is_image(&file){
                continue;
            }
            let size = match size(file.path()){
                Ok(size) => {size},
                Err(_) => {continue;}
            };

            if !images_shapes.contains_key(&size){
                *images_shapes.entry(size).or_insert(0) +=  1;
                images_with_shape.insert(size, vec![(folder_name.clone(), file_name.clone())]);
            }
            else{
                *images_shapes.entry(size).or_insert(0) +=  1;
                let mut new_vec = images_with_shape[&size].clone();
                new_vec.push((folder_name.clone(), file_name.clone()));
                images_with_shape.insert(size, new_vec);
            }
        }
    }
    images_shapes.sort_keys();
    return Ok((images_shapes, images_with_shape))

}