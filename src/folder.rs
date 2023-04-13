use std::collections::LinkedList;
use std::fs;
use std::path::{Path, PathBuf};

use crate::copier::Copier;
use crate::copier::FileToCopy;
pub struct Folder {
    pub name: String,
    pub files: Vec<PathBuf>,
    total_size: usize,
}
impl Folder {
    pub fn from_path(path: &str) -> Self {
        let files: Vec<PathBuf> = Vec::new();
        Self {
            name: String::from(path),
            files,
            total_size: 0,
        }
    }
    pub fn load_files_from_path(&mut self) {
        let mut path_list: LinkedList<PathBuf> = LinkedList::new();
        let path_buf = PathBuf::from(&self.name);
        path_list.push_back(path_buf);
        while !path_list.is_empty() {
            let path_buf = path_list.pop_back().unwrap();
            let paths = fs::read_dir(path_buf).expect("invalid path");
            for path in paths {
                let path = path.expect("invalid path").path();
                if path.is_dir() {
                    path_list.push_back(path);
                } else if path.is_file() {
                    let file_size = path.metadata().unwrap().len() as usize;
                    self.total_size += file_size;
                    self.files.push(path);
                }
            }
        }
    }
    pub fn copy_to(&self, path: &str) {
        let path = PathBuf::from(path);
        let file_in_target_dir = self
            .files
            .clone()
            .into_iter()
            .map(|f| FileToCopy::from_files(path.join(self.get_file_path_from_folder(&f)), f))
            .rev()
            .collect();

        let mut copier = Copier::from_folder_to_dir(&file_in_target_dir, self.total_size);
        copier.copy();
    }
    fn get_file_path_from_folder(&self, file: &Path) -> PathBuf {
        let parent_path = Path::new(&self.name)
            .parent()
            .expect("doestn have a parent");
        let file_whithout_path = file
            .strip_prefix(parent_path)
            .expect("is not a prefix of the file");
        file_whithout_path.to_path_buf()
    }
}
