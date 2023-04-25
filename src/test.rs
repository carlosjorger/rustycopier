// use super::*;

use std::{
    env::temp_dir,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use tempdir::TempDir;

use crate::folder_to_copy::FolderToCopy;

#[test]
fn copy_one_file_in_a_folder() {
    let source_dir = TempDir::new("my_source_dir").expect("unable create a dir");

    let file_source_path: std::path::PathBuf = source_dir.path().join("poetry.txt");

    let msg = b"In the world of coding, a language stands
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
                            For a language that's powerful and right.";
    create_file(&file_source_path, msg);

    let source_dir_str = source_dir.path().to_str().unwrap();

    let mut folder: FolderToCopy = FolderToCopy::from_path(source_dir_str);
    folder.load_files_from_path();

    let destiny_temp_dir = TempDir::new("my_destiny_dir").expect("unable create a dir");
    folder.copy_to(destiny_temp_dir.path().to_str().unwrap());

    let file_destiny_path: std::path::PathBuf = destiny_temp_dir
        .path()
        .join(file_source_path.parent().unwrap().file_name().unwrap())
        .join("poetry.txt");
    let new_msg = fs::read(&file_destiny_path).unwrap();
    assert_eq!(msg.to_vec(), new_msg);
}

fn create_file(file_path: &PathBuf, content: &[u8]) {
    let mut file_source = File::create(file_path).expect("unable create a new file");

    file_source
        .write_all(content)
        .expect("unable to write in this temp file");
}
#[test]
fn copy_one_file() {
    let source_dir = temp_dir();

    let file_source_path: std::path::PathBuf = source_dir.join("poetry.txt");

    let msg = b"In the world of coding, a language stands
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
                            For a language that's powerful and right.";
    create_file(&file_source_path, msg);

    let source_dir_str = file_source_path.to_str().unwrap();

    let mut folder: FolderToCopy = FolderToCopy::from_path(source_dir_str);
    folder.load_files_from_path();

    let destiny_temp_dir = TempDir::new("my_destiny_dir").expect("unable create a dir");
    folder.copy_to(destiny_temp_dir.path().to_str().unwrap());

    let file_destiny_path: std::path::PathBuf = destiny_temp_dir
        .path()
        .join(file_source_path.file_name().unwrap());
    let new_msg = fs::read(&file_destiny_path).unwrap();
    assert_eq!(msg.to_vec(), new_msg);
}
