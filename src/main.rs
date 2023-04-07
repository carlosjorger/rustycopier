mod progress_bar;

use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::{env, time::Instant};

struct Folder {
    name: String,
    sub_folders: Vec<Folder>,
    files: Vec<File>,
}
impl Folder {
    fn from_path(path: &str) -> Self {
        let paths = fs::read_dir(path).expect("invalid path");
        let mut sub_folders: Vec<Folder> = Vec::new();
        let mut files: Vec<File> = Vec::new();

        for path in paths {
            let path = path.expect("invalid path").path();
            if path.is_dir() {
                let dir = Folder::from_path(path.to_str().expect("invalid dir"));
                sub_folders.push(dir);
            } else if path.is_file() {
                let file = File::open(path.to_str().unwrap()).expect("invalid file");
                files.push(file);
            }
        }
        Self {
            name: String::from(path),
            files,
            sub_folders,
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please pass the origin path");
        return;
    }
    if args.len() <= 2 {
        println!("Please pass the destiny path");
        return;
    }

    let source = &args[1];
    let destiny = &args[2];
    // let folder = Folder::from_path(&destiny);
    let file_name = &get_file_name_from_path(source);
    let destiny_with_file = add_file_name_to_path(destiny, file_name);
    let start = Instant::now();
    copy(&source, &destiny_with_file, 1024 * 1000);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
fn get_file_name_from_path(path: &str) -> String {
    let path = Path::new(&path);
    let file_name = path
        .file_name()
        .expect("the origin path is not a valid file")
        .to_str()
        .unwrap();
    file_name.to_string()
}
fn add_file_name_to_path(path: &str, file_name: &str) -> String {
    let path = Path::new(&path);
    let path_with_file_name = path.join(file_name);
    let path_str = path_with_file_name.to_str().unwrap();
    path_str.to_string()
}
fn copy(source_path: &str, destiny_path: &str, capacity: usize) {
    let destiny_file =
        File::create(destiny_path).expect("Should have been able to read the destiny path");
    let source_file =
        File::open(source_path).expect("Should have been able to read the source path");

    let mut stream = BufWriter::with_capacity(capacity, &destiny_file);
    let mut reader = BufReader::with_capacity(capacity, &source_file);

    let size = source_file.metadata().unwrap().len() as usize;
    let mut progress = progress_bar::ProgressBar::from_total_size(size);
    println!("");
    loop {
        let buffer = reader.fill_buf().expect("error in the buffer");
        let lenght = buffer.len();
        stream.write(buffer).expect("error to write");
        reader.consume(lenght);
        progress.consume(lenght);
        if lenght == 0 {
            break;
        }
    }
    println!()
}
