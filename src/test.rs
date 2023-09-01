use crate::copier::FileToCopy;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{
    env::temp_dir,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};
use tempdir::TempDir;

fn create_file(file_path: &PathBuf, content: &[u8]) -> Result<(), io::Error> {
    let mut file_source = File::create(file_path)?;

    file_source.write_all(content)?;
    Ok(())
}
fn copy_to_path(source: &str, target_path: &Path) {
    let mut folder: FileToCopy = FileToCopy::from_path(source);
    folder.load_files_from_path();

    folder.copy_to(target_path);
}

fn get_msg_from_file(dir: &TempDir, file_path: &Path) -> Vec<u8> {
    let file_destiny_path: std::path::PathBuf = dir.path().join(file_path.file_name().unwrap());
    fs::read(file_destiny_path).unwrap()
}
#[test]
fn copy_one_file_in_a_folder() {
    let source_dir = TempDir::new("my_source_dir").expect("unable create a dir");

    let file_source_path: std::path::PathBuf = source_dir.path().join("poetry.txt");

    let msg = get_example_text();
    create_file(&file_source_path, msg).expect("unable to create the file");

    let source_dir_str = source_dir.path().to_str().unwrap();

    let destiny_temp_dir = TempDir::new("my_destiny_dir").expect("unable create a dir");

    copy_to_path(source_dir_str, destiny_temp_dir.path());

    let file_destiny_path: std::path::PathBuf = destiny_temp_dir
        .path()
        .join(file_source_path.parent().unwrap().file_name().unwrap())
        .join("poetry.txt");
    let new_msg = fs::read(file_destiny_path).unwrap();
    assert_eq!(msg.to_vec(), new_msg);
}

#[test]
fn copy_one_file() {
    let source_dir = temp_dir();

    let file_source_path: std::path::PathBuf = source_dir.join("poetry.txt");

    let msg = get_example_text();
    create_file(&file_source_path, msg).unwrap();

    let source_dir_str = file_source_path.to_str().unwrap();
    let destiny_temp_dir = TempDir::new("my_destiny_dir").expect("unable create a dir");

    copy_to_path(source_dir_str, destiny_temp_dir.path());

    let new_msg = get_msg_from_file(&destiny_temp_dir, &file_source_path);

    assert_eq!(msg.to_vec(), new_msg);
}
#[test]
#[ignore]
fn copy_10000_files() {
    const NUMBER_OF_FILES: usize = 10000;
    let source_dir = TempDir::new("my_source_dir").expect("unable create a dir");
    let msg = get_example_text();
    for file_number in 0..NUMBER_OF_FILES {
        let file_source_path: std::path::PathBuf =
            source_dir.path().join(format!("poetry{}.txt", file_number));

        create_file(&file_source_path, msg).unwrap();
    }
    let source_dir_str = source_dir.path().to_str().unwrap();
    let destiny_temp_dir = TempDir::new("my_destiny_dir").expect("unable create a dir");

    copy_to_path(source_dir_str, destiny_temp_dir.path());

    let msg_vector = msg.to_vec();
    (0..NUMBER_OF_FILES)
        .into_par_iter()
        .for_each(|file_number| {
            let file_destiny_path: std::path::PathBuf = destiny_temp_dir
                .path()
                .join(source_dir.path().file_name().unwrap())
                .join(format!("poetry{}.txt", file_number));
            let new_msg = fs::read(file_destiny_path).unwrap();
            assert_eq!(msg_vector, new_msg);
        });
}

#[test]
#[ignore]
fn copy_5000_files() {
    const NUMBER_OF_FILES: usize = 5000;
    let source_dir = TempDir::new("my_source_dir").expect("unable create a dir");
    let msg = get_example_text();
    for file_number in 0..NUMBER_OF_FILES {
        let file_source_path: std::path::PathBuf =
            source_dir.path().join(format!("poetry{}.txt", file_number));

        create_file(&file_source_path, msg).unwrap();
    }
    let source_dir_str = source_dir.path().to_str().unwrap();
    let destiny_temp_dir = TempDir::new("my_destiny_dir").expect("unable create a dir");

    copy_to_path(source_dir_str, destiny_temp_dir.path());
    let msg_vector = msg.to_vec();
    (0..NUMBER_OF_FILES)
        .into_par_iter()
        .for_each(|file_number| {
            let file_destiny_path: std::path::PathBuf = destiny_temp_dir
                .path()
                .join(source_dir.path().file_name().unwrap())
                .join(format!("poetry{}.txt", file_number));
            let new_msg = fs::read(file_destiny_path).unwrap();
            assert_eq!(msg_vector, new_msg);
        });
}
fn get_example_text<'a>() -> &'a [u8; 1037] {
    b"In the world of coding, a language stands
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
