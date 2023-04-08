mod progress_bar;

use std::collections::LinkedList;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::{env, time::Instant};

struct Folder {
    name: String,
    files: Vec<PathBuf>,
}
impl Folder {
    fn from_path(path: &str) -> Self {
        let files: Vec<PathBuf> = Vec::new();
        Self {
            name: String::from(path),
            files,
        }
    }
    fn load_files_from_path(&mut self) {
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
    fn copy_to(path: &str) {
        todo!()
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
    let mut folder = Folder::from_path(destiny);
    folder.load_files_from_path();
    let file_name = &get_file_name_from_path(source);
    let destiny_with_file = add_file_name_to_path(destiny, file_name);
    let start = Instant::now();
    copy(source, &destiny_with_file, 1024 * 1000);
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
    println!();
    loop {
        let buffer = reader.fill_buf().expect("error in the buffer");
        let lenght = buffer.len();
        stream.write_all(buffer).expect("error to write");
        reader.consume(lenght);
        progress.consume(lenght);
        if lenght == 0 {
            break;
        }
    }
    println!()
}
