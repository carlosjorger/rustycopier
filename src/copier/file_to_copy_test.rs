use std::{fs::create_dir, path::Path};

use assert_fs::prelude::{FileWriteStr, PathAssert, PathChild};

use super::FileToCopy;

#[test]
fn copy_one_file_in_a_folder() {
    let temp_root = assert_fs::TempDir::new().unwrap();
    let file = temp_root.child("poetry.txt");
    let poetry = get_random_poetry();
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
    let poetry = get_random_poetry();
    file.write_str(poetry).unwrap();
    let target_folder = temp_root.child("my_targer_folder");
    create_dir(&target_folder).unwrap();
    let source_folder = temp_root.child("my_source_folder");
    copy_to_path(&source_folder, &target_folder);
    let copied_file = target_folder.child("my_source_folder/poetry.txt");
    copied_file.assert(poetry);
}
#[test]
fn copy_a_nested_folder_in_a_folder() {
    let temp_root = assert_fs::TempDir::new().unwrap();
    let file = temp_root.child("my_source_folder/poetries/love/poetry.txt");
    let poetry = get_random_poetry();
    file.write_str(poetry).unwrap();
    let temp_folder = temp_root.child("my_targer_folder");
    create_dir(&temp_folder).unwrap();
    let source_folder = temp_root.child("my_source_folder");
    copy_to_path(&source_folder, &temp_folder);
    let copied_file = temp_folder.child("my_source_folder/poetries/love/poetry.txt");
    copied_file.assert(poetry);
}
fn get_random_poetry<'a>() -> &'a str {
    "In the world of coding, a language stands
                            Rust, they call it, with its own demands
                            It's strict and crotchety, some may say
                            But its power and speed are here to stay

                            Its syntax may seem a bit obtuse
                            But its memory safety is no excuse
                            For sloppy code that could bring down
                            An entire system with just one frown

                            Rust is compiled, not interpreted
                            And its performance can't be debated
                            It's perfect for systems and low-level tasks
                            And its community is growing fast

                            So if you're feeling crotchety today
                            Give Rust a chance, don't turn away
                            It may be strict, but it's worth the fight
                            For a language that's powerful and right."
}

fn copy_to_path(source: &Path, target_path: &Path) {
    let source_path = &source.to_path_buf();
    let mut folder: FileToCopy = FileToCopy::from_path(source_path);
    folder.load_files_from_path().unwrap();
    folder.copy_to(target_path);
}
