#[cfg(test)]
use super::FileToCopy;
use crate::utils::test_utils;
use assert_fs::prelude::{FileWriteStr, PathAssert, PathChild};
use std::{fs::create_dir, path::Path};
#[test]
fn copy_one_file_in_a_folder() {
    let temp_root = assert_fs::TempDir::new().unwrap();
    let file = temp_root.child("poetry.txt");
    let poetry = test_utils::get_random_poetry();
    file.write_str(poetry).unwrap();
    let temp_folder = temp_root.child("my_targer_folder");
    create_dir(&temp_folder).unwrap();
    copy_to_path(&file, &temp_folder);
    let copied_file = temp_folder.child("poetry.txt");
    copied_file.assert(poetry);
}
#[test]
fn copy_one_folder_in_a_folder() {
    let temp_root = assert_fs::TempDir::new().unwrap();
    let file = temp_root.child("my_source_folder/poetry.txt");
    let poetry = test_utils::get_random_poetry();
    file.write_str(poetry).unwrap();
    let target_folder = test_utils::create_temp_child_folder(&temp_root, "my_targer_folder");
    let source_folder = temp_root.child("my_source_folder");
    copy_to_path(&source_folder, &target_folder);
    let copied_file = target_folder.child("my_source_folder/poetry.txt");
    copied_file.assert(poetry);
}
#[test]
fn copy_a_nested_folder_in_a_folder() {
    let temp_root = assert_fs::TempDir::new().unwrap();
    let file = temp_root.child("my_source_folder/poetries/love/poetry.txt");
    let poetry = test_utils::get_random_poetry();
    file.write_str(poetry).unwrap();
    let temp_folder = temp_root.child("my_targer_folder");
    create_dir(&temp_folder).unwrap();
    let source_folder = temp_root.child("my_source_folder");
    copy_to_path(&source_folder, &temp_folder);
    let copied_file = temp_folder.child("my_source_folder/poetries/love/poetry.txt");
    copied_file.assert(poetry);
}
fn copy_to_path(source: &Path, target_path: &Path) {
    let source_path = &source.to_path_buf();
    let mut folder: FileToCopy = FileToCopy::from_path(source_path);
    folder.load_files_from_path().unwrap();
    folder.copy_to(target_path, true);
}
