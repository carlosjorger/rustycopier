mod progress_bar;

use std::collections::{linked_list, LinkedList};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::{env, time::Instant};

struct Folder {
    name: String,
    // sub_folders: Vec<Folder>,
    files: Vec<PathBuf>,
}
impl Folder {
    fn from_path(path: &str) -> Self {
        let mut files: Vec<PathBuf> = Vec::new();
        let mut linked_list: LinkedList<PathBuf> = LinkedList::new();
        let path_buf = PathBuf::from(path);
        linked_list.push_back(path_buf);
        while linked_list.len() > 0 {
            let path_buf = linked_list.pop_back().unwrap();
            let paths = fs::read_dir(path_buf).expect("invalid path");
            for path in paths {
                let path = path.expect("invalid path").path();
                if path.is_dir() {
                    linked_list.push_back(path);
                    // let path = path.to_str().expect("invalid dir");
                    // sub_folders.push(dir);
                } else if path.is_file() {
                    files.push(path);
                }
            }
        }
        Self {
            name: String::from(path),
            files,
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
    let folder = Folder::from_path(&destiny);

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
