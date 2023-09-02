use crate::copier::FileCopy;
use std::collections::LinkedList;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};
use std::process;

use super::copier::Copier;

pub struct FileToCopy<'a> {
    path: &'a Path,
    file_paths: Vec<PathBuf>,
    total_size: usize,
    parent_path: Option<&'a Path>,
}
impl<'a> FileToCopy<'a> {
    pub fn from_path(path: &'a str) -> Self {
        let file_paths: Vec<PathBuf> = Vec::new();
        let parent_path = Path::new(path).parent();
        Self {
            path: Path::new(path),
            file_paths,
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
            self.read_subdirs_from_path(path_buf, &mut path_list);
        }
    }
    fn read_subdirs_from_path(&mut self, path: PathBuf, path_list: &mut LinkedList<PathBuf>) {
        match fs::read_dir(&path) {
            Ok(paths) => {
                for path in paths {
                    let path = path.expect("invalid path").path();
                    if path.is_dir() {
                        path_list.push_back(path);
                    } else if path.is_file() {
                        self.save_file(path);
                    }
                }
            }
            Err(_) => {
                if let Some(path_name) = path.to_str() {
                    eprintln!("!!!!The path {path_name} doesn´t exist");
                    process::exit(1);
                }
            }
        }
    }
    fn save_file(&mut self, file_path: PathBuf) {
        let file_size = file_path.metadata().unwrap().len() as usize;
        self.total_size += file_size;
        self.file_paths.push(file_path);
    }
    pub fn copy_to(&mut self, target_path: &Path) {
        self.create_source_folder(target_path);

        let copies = self.file_paths.iter().map(|path: &PathBuf| {
            FileCopy::from_files(
                self.get_path_with_prefix_path(path, target_path),
                path.to_path_buf(),
            )
        });

        let mut copier = Copier::from_folder_to_dir();
        copier.start(copies);
    }
    fn create_source_folder(&self, target_path: &Path) {
        if self.path.is_dir() {
            create_dir_all(self.get_path_with_prefix_path(self.path, target_path))
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
