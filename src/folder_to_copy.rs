use std::collections::LinkedList;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};

use crate::copier::Copier;
use crate::copier::FileCopy;
pub struct FolderToCopy<'a> {
    path: &'a Path,
    pub files: Vec<PathBuf>,
    total_size: usize,
    parent_path: Option<&'a Path>,
}
impl<'a> FolderToCopy<'a> {
    pub fn from_path(path: &'a str) -> Self {
        let files: Vec<PathBuf> = Vec::new();
        let parent_path = Path::new(path).parent();
        Self {
            path: Path::new(path),
            files,
            total_size: 0,
            parent_path,
        }
    }
    pub fn load_files_from_path(&mut self) {
        let mut path_list: LinkedList<PathBuf> = LinkedList::new();
        let path_buf = PathBuf::from(&self.path);
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
    pub fn copy_to(&self, target_path: &str) {
        let target_path = PathBuf::from(target_path);
        self.create_source_folder(&target_path);
        let file_in_target_dir = self
            .files
            .clone()
            .into_iter()
            .map(|f| FileCopy::from_files(target_path.join(self.get_file_path_from_folder(&f)), f))
            .rev()
            .collect();

        let mut copier = Copier::from_folder_to_dir(&file_in_target_dir, self.total_size);
        copier.start();
    }
    fn create_source_folder(&self, target_path: &PathBuf) {
        if self.path.is_dir() {
            create_dir_all(self.get_path_with_prefix_path(&self.path.to_path_buf(), target_path))
                .expect("error creating the folder");
        }
    }
    fn get_path_with_prefix_path(&self, path: &PathBuf, prefix_path: &PathBuf) -> PathBuf {
        prefix_path.join(self.get_file_path_from_folder(path))
    }
    fn get_file_path_from_folder(&self, file: &Path) -> PathBuf {
        if let Some(parent) = self.parent_path {
            let file_whithout_path = file
                .strip_prefix(parent)
                .expect("is not a prefix of the file");
            return file_whithout_path.to_path_buf();
        }
        file.to_path_buf()
    }
}
