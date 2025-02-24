/*
    floppy_manager.rc
    Enumerate images in the /floppy directory to allow floppy selection

*/

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    ffi::OsString,
    fs,
    error::Error,
    fmt::Display
};

#[derive(Debug)]
pub enum FloppyError {
    DirNotFound,
    FileReadError,
}
impl Error for FloppyError {}
impl Display for FloppyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FloppyError::DirNotFound => write!(f, "Couldn't find the requested directory."),
            FloppyError::FileReadError => write!(f, "A file read error occurred."),
        }
    }
}

#[allow(dead_code)]
pub struct FloppyImage {
    path: PathBuf,
    size: u64
}

pub struct FloppyManager {
    image_vec: Vec<FloppyImage>,
    image_map: HashMap<OsString, FloppyImage>
}

impl FloppyManager {
    pub fn new() -> Self {
        Self {
            image_vec: Vec::new(),
            image_map: HashMap::new()
        }
    }

    pub fn scan_dir(&mut self, path: &Path) -> Result<bool, FloppyError> {

        // Read in directory entries within the provided path
        let dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(_) => return Err(FloppyError::DirNotFound)
        };

        let extensions = ["img", "ima"];

        // Clear and rebuild image lists.
        self.image_vec.clear();
        self.image_map.clear();

        // Scan through all entries in the directory and find all files with matching extension
        for entry in dir {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    if let Some(extension) = entry.path().extension() {
                        if extensions.contains(&extension.to_string_lossy().to_lowercase().as_ref()) {

                            println!("Found floppy image: {:?} size: {}", entry.path(), entry.metadata().unwrap().len());
                            
                            self.image_vec.push( 
                                FloppyImage {
                                    path: entry.path(),
                                    size: entry.metadata().unwrap().len()
                                }
                            );
                        
                            self.image_map.insert(entry.file_name(), 
                                FloppyImage { 
                                    path: entry.path(),
                                    size: entry.metadata().unwrap().len()
                                 }
                            );
                        }
                    }
                }
            }
        }
        Ok(true)
    }


    pub fn get_floppy_names(&self) -> Vec<OsString> {
        let mut vec: Vec<OsString> = Vec::new();
        for (key, _val) in &self.image_map {
            vec.push(key.clone());
        }
        vec.sort_by(|a, b| a.to_ascii_uppercase().cmp(&b.to_ascii_uppercase()));
        vec
    }

    pub fn load_floppy_data(&self, name: &OsString ) -> Result<Vec<u8>, FloppyError> {

        let mut floppy_vec = Vec::new();
        if let Some(floppy) = self.image_map.get(name) {
            floppy_vec = match std::fs::read(&floppy.path) {
                Ok(vec) => vec,
                Err(e) => {
                    eprintln!("Couldn't open floppy image: {}", e);
                    return Err(FloppyError::FileReadError);
                }
            };
        }

        Ok(floppy_vec)
    }

}
