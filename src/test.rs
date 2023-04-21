// use super::*;

use std::{
    fs::{self, File},
    io::Write,
};

use tempdir::TempDir;

use crate::folder::Folder;

#[test]
fn copy_one_file() {
    let source_dir = TempDir::new("my_source_dir").expect("unable create a dir");

    let file_source_path: std::path::PathBuf = source_dir.path().join("poetry.txt");

    let mut file_source = File::create(file_source_path).expect("unable create a new file");

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
    file_source
        .write_all(msg)
        .expect("unable to write in this temp file");

    let source_dir_str=source_dir.path().to_str().unwrap();

    let folder: Folder = Folder::from_path(source_dir_str);
    let paths = fs::read_dir(source_dir.path()).expect("invalid path");
    
    let destiny_temp_dir = TempDir::new("my_destiny_dir").expect("unable create a dir");
    folder.copy_to(destiny_temp_dir.path().to_str().unwrap());

    let file_destiny_path: std::path::PathBuf = destiny_temp_dir.path().join("poetry.txt");
    let new_file = File::open(file_destiny_path).expect("unable open the file");

    // assert_eq!(4, 2);

    // source_dir.close().expect("error removing the temp dir");
}
