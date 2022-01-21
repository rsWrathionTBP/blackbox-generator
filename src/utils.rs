use std::fs;

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