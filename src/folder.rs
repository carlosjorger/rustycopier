use std::collections::LinkedList;
use std::fs::{self};
use std::path::{Path, PathBuf};

use crate::copier::Copier;

pub struct Folder {
    pub name: String,
    pub files: Vec<PathBuf>,
}
impl Folder {
    pub fn from_path(path: &str) -> Self {
        let files: Vec<PathBuf> = Vec::new();
        Self {
            name: String::from(path),
            files,
        }
    }
    pub fn load_files_from_path(&mut self) {
        let mut linked_list: LinkedList<PathBuf> = LinkedList::new();
        let path_buf = PathBuf::from(&self.name);
        linked_list.push_back(path_buf);
        while !linked_list.is_empty() {
            let path_buf = linked_list.pop_back().unwrap();
            let paths = fs::read_dir(path_buf).expect("invalid path");
            for path in paths {
                let path = path.expect("invalid path").path();
                if path.is_dir() {
                    linked_list.push_back(path);
                } else if path.is_file() {
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
            .map(|f| path.join(self.get_file_path_from_folder(&f)))
            .rev()
            .collect();

        let mut copier = Copier::from_folder_to_dir(&file_in_target_dir);
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
