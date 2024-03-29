#[cfg(test)]
use super::FileToCopy;
use crate::utils::test_utils;
use assert_fs::prelude::{PathAssert, PathChild};
use std::path::Path;
#[test]
fn copy_one_file_in_a_folder() {
    let temp_root = assert_fs::TempDir::new().unwrap();
    let poetry = test_utils::get_random_poetry();
    let file = test_utils::create_temp_child_file(&temp_root, "poetry.txt", poetry);
    let temp_folder = test_utils::create_temp_child_folder(&temp_root, "my_targer_folder");
    copy_to_path(&file, &temp_folder, false);
    let copied_file = temp_folder.child("poetry.txt");
    copied_file.assert(poetry);
}
#[test]
fn copy_one_folder_in_a_folder() {
    let temp_root = assert_fs::TempDir::new().unwrap();
    let poetry = test_utils::get_random_poetry();
    let _ = test_utils::create_temp_child_file(&temp_root, "my_source_folder/poetry.txt", poetry);
    let target_folder = test_utils::create_temp_child_folder(&temp_root, "my_targer_folder");
    let source_folder = temp_root.child("my_source_folder");
    copy_to_path(&source_folder, &target_folder, false);
    let copied_file = target_folder.child("my_source_folder/poetry.txt");
    copied_file.assert(poetry);
}
#[test]
fn copy_a_nested_folder_in_a_folder() {
    let temp_root = assert_fs::TempDir::new().unwrap();
    let poetry = test_utils::get_random_poetry();
    let _ = test_utils::create_temp_child_file(
        &temp_root,
        "my_source_folder/poetries/love/poetry.txt",
        poetry,
    );
    let temp_folder = test_utils::create_temp_child_folder(&temp_root, "my_targer_folder");
    let source_folder = temp_root.child("my_source_folder");
    copy_to_path(&source_folder, &temp_folder, false);
    let copied_file = temp_folder.child("my_source_folder/poetries/love/poetry.txt");
    copied_file.assert(poetry);
}
pub fn copy_to_path(source: &Path, target_path: &Path, is_logging_active: bool) {
    let source_path = &source.to_path_buf();
    let mut folder: FileToCopy = FileToCopy::from_path(source_path);
    folder.load_files_from_path().unwrap();
    folder.copy_to(target_path, is_logging_active);
}
