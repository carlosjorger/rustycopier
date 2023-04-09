use std::collections::LinkedList;
use std::fs::{self, create_dir_all, File};
use std::path::{Path, PathBuf};
struct Copier<'a> {
    folder: &'a Folder,
    path: &'a str,
}
impl<'a> Copier<'a> {
    fn from_folder_to_dir(folder: &'a Folder, path: &'a str) -> Self {
        Self { folder, path }
    }
    fn copy(&self) {
        for file in &self.folder.files {
            self.create_file(self.path, file);
        }
    }
    fn create_file(&self, path: &str, file: &Path) {
        let file_path_from_folder = self.get_file_path_from_folder(file);
        let destiny_path = Path::new(path).join(file_path_from_folder);
        let destiny_parent_path = destiny_path.parent().unwrap();
        create_dir_all(destiny_parent_path).unwrap_or_else(|e| {
            panic!(
                "Error creating dir {}: {:?}",
                destiny_parent_path.to_str().unwrap(),
                e
            )
        });

        let _ = File::create(&destiny_path).unwrap_or_else(|e| {
            panic!(
                "Error creating file {}: {:?}",
                destiny_path.to_str().unwrap(),
                e
            )
        });
    }
    fn get_file_path_from_folder(&'a self, file: &'a Path) -> &Path {
        let parent_path = Path::new(&self.folder.name)
            .parent()
            .expect("doestn have a parent");
        let file_whithout_path = file
            .strip_prefix(parent_path)
            .expect("is not a prefix of the file");
        file_whithout_path
    }
}
pub struct Folder {
    name: String,
    files: Vec<PathBuf>,
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
        let copier = Copier::from_folder_to_dir(self, path);
        copier.copy();
    }
}
