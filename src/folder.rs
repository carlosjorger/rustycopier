use std::collections::LinkedList;
use std::fs;
use std::path::{Path, PathBuf};

use crate::copier::Copier;
use crate::copier::FileCopy;
pub struct Folder<'a> {
    pub name: String,
    pub files: Vec<PathBuf>,
    total_size: usize,
    parent_path: &'a Path,
}
impl<'a> Folder<'a> {
    pub fn from_path(path: &'a str) -> Self {
        let files: Vec<PathBuf> = Vec::new();
        let parent_path = Path::new(path).parent().expect("doestn have a parent");
        Self {
            name: String::from(path),
            files,
            total_size: 0,
            parent_path,
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
            .map(|f| FileCopy::from_files(path.join(self.get_file_path_from_folder(&f)), f))
            .rev()
            .collect();

        let mut copier = Copier::from_folder_to_dir(&file_in_target_dir, self.total_size);
        copier.start();
    }
    fn get_file_path_from_folder(&self, file: &Path) -> PathBuf {
        let file_whithout_path = file
            .strip_prefix(self.parent_path)
            .expect("is not a prefix of the file");
        file_whithout_path.to_path_buf()
    }
}
