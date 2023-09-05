use anyhow::{Context, Error, Ok};

use crate::copier::FileCopy;
use std::collections::LinkedList;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};

use super::Copier;

pub struct FileToCopy<'a> {
    path: PathBuf,
    file_paths: Vec<PathBuf>,
    total_size: usize,
    parent_path: Option<&'a Path>,
}
impl<'a> FileToCopy<'a> {
    pub fn from_path(path: &'a PathBuf) -> Self {
        let file_paths: Vec<PathBuf> = Vec::new();
        let parent_path = path.parent();
        Self {
            path: PathBuf::from(path),
            file_paths,
            total_size: 0,
            parent_path,
        }
    }
    pub fn load_files_from_path(&mut self) -> Result<(), Error> {
        let mut path_list: LinkedList<PathBuf> = LinkedList::new();
        let path_buf = PathBuf::from(&self.path);
        if path_buf.is_file() {
            self.save_file(path_buf);
            return Ok(());
        }
        path_list.push_back(path_buf);
        while let Some(path_buf) = path_list.pop_back() {
            self.read_subdirs_from_path(path_buf, &mut path_list)?;
        }
        Ok(())
    }
    fn read_subdirs_from_path(
        &mut self,
        path: PathBuf,
        path_list: &mut LinkedList<PathBuf>,
    ) -> Result<(), Error> {
        let paths = fs::read_dir(&path)
            .with_context(|| (format!("Could not read file the path`{}`", path.display())))?;

        for path in paths {
            let path = path.expect("invalid path").path();
            if path.is_dir() {
                path_list.push_back(path);
            } else if path.is_file() {
                self.save_file(path);
            }
        }
        Ok(())
    }
    fn save_file(&mut self, file_path: PathBuf) {
        let file_size = file_path.metadata().unwrap().len() as usize;
        self.total_size += file_size;
        self.file_paths.push(file_path);
    }
    pub fn copy_to(&mut self, target_path: &Path, is_logging_active: bool) {
        self.create_source_folder(target_path);
        let copies = self.file_paths.iter().map(|path: &PathBuf| {
            let target_path = self.get_file_path_inside_folder(path, target_path);
            self.create_parent_folder(&target_path);
            FileCopy::from_files(target_path, path.to_path_buf()).unwrap()
        });
        let mut copier = Copier::from_folder_to_dir();
        copier.start(copies, is_logging_active);
    }
    fn create_parent_folder(&self, file_path: &Path) {
        if let Some(parent_folder) = file_path.parent() {
            if !parent_folder.exists() {
                create_dir_all(parent_folder).expect("error creating the folder");
            }
        }
    }
    fn create_source_folder(&self, target_path: &Path) {
        if self.path.is_dir() {
            create_dir_all(self.get_file_path_inside_folder(&self.path, target_path))
                .expect("error creating the folder");
        }
    }
    fn get_file_path_inside_folder(&self, file_path: &Path, folder_path: &Path) -> PathBuf {
        folder_path.join(self.get_file_path_from_source_folder(file_path))
    }
    fn get_file_path_from_source_folder(&self, file: &Path) -> PathBuf {
        if let Some(parent) = self.parent_path {
            let file_whithout_path = file
                .strip_prefix(parent)
                .expect("is not a prefix of the file");
            return file_whithout_path.to_path_buf();
        }
        file.to_path_buf()
    }
}
