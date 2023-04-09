use std::{
    fs::{create_dir_all, File},
    io,
    path::{Path, PathBuf},
};

pub struct Copier<'a> {
    files: &'a Vec<PathBuf>,
    paused: bool,
}
impl<'a> Copier<'a> {
    pub fn from_folder_to_dir(files: &'a Vec<PathBuf>) -> Self {
        Self {
            files,
            paused: false,
        }
    }
    pub fn copy(&mut self) {
        for file in self.files {
            if !self.paused {
                self.create_file(file).expect("error copy the file");
                self.paused = false;
            }
        }
    }
    fn create_file(&self, file: &Path) -> Result<(), io::Error> {
        let file_folder = file.parent().expect("doestn have a parent");
        create_dir_all(file_folder)?;
        let _ = File::create(&file)?;
        Ok(())
    }
}
