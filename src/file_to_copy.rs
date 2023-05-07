use std::collections::LinkedList;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};

use crate::copier::Copier;
use crate::copier::FileCopy;
pub struct FileToCopy<'a> {
    path: &'a Path,
    files: Vec<PathBuf>,
    total_size: usize,
    parent_path: Option<&'a Path>,
}
impl<'a> FileToCopy<'a> {
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
        if path_buf.is_file() {
            self.save_file(path_buf);
            return;
        }
        path_list.push_back(path_buf);
        while let Some(path_buf) = path_list.pop_back() {
            let paths = fs::read_dir(path_buf).expect("invalid path");
            for path in paths {
                let path = path.expect("invalid path").path();
                if path.is_dir() {
                    path_list.push_back(path);
                } else if path.is_file() {
                    self.save_file(path);
                }
            }
        }
    }
    fn save_file(&mut self, file_path: PathBuf) {
        let file_size = file_path.metadata().unwrap().len() as usize;
        self.total_size += file_size;
        self.files.push(file_path);
    }
    pub fn copy_to(&mut self, target_path: &Path) {
        self.create_source_folder(&target_path);

        let file_in_target_dir = self
            .files
            .to_owned()
            .into_iter()
            .map(|f| FileCopy::from_files(self.get_path_with_prefix_path(&f, &target_path), f));

        let mut copier = Copier::from_folder_to_dir();
        copier.start(file_in_target_dir);
    }
    fn create_source_folder(&self, target_path: &Path) {
        if self.path.is_dir() {
            create_dir_all(self.get_path_with_prefix_path(&self.path, target_path))
                .expect("error creating the folder");
        }
    }
    fn get_path_with_prefix_path(&self, path: &Path, prefix_path: &Path) -> PathBuf {
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
