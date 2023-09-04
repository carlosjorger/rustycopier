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
// #[test]
// fn copy_one_file_in_a_folder() {
//     let source_dir = TempDir::new("my_source_dir").expect("unable create a dir");

//     let file_source_path: std::path::PathBuf = source_dir.path().join("poetry.txt");

//     let msg = get_example_text();
//     create_file(&file_source_path, msg).expect("unable to create the file");

//     let source_dir_str = source_dir.path();

//     let destiny_temp_dir = TempDir::new("my_destiny_dir").expect("unable create a dir");

//     copy_to_path(source_dir_str, destiny_temp_dir.path());

//     let file_destiny_path: std::path::PathBuf = destiny_temp_dir
//         .path()
//         .join(file_source_path.parent().unwrap().file_name().unwrap())
//         .join("poetry.txt");
//     let new_msg = fs::read(file_destiny_path).unwrap();
//     assert_eq!(msg.to_vec(), new_msg);
// }

fn copy_to_path(source: &Path, target_path: &Path) {
    let source_path = &source.to_path_buf();
    let mut folder: FileToCopy = FileToCopy::from_path(source_path);
    folder.load_files_from_path().unwrap();
    folder.copy_to(target_path);
}
