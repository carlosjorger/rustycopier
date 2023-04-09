// mod progress_bar;

use std::collections::LinkedList;
use std::fs::{self, create_dir_all, File};
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
    fn copy_to(&self, path: &str) {
        for file in &self.files {
            self.create_file(path, file);
        }
    }
    fn create_file(&self, path: &str, file: &Path) {
        let file_path_from_folder = self.get_file_path_from_folder(file);
        let destiny_path = Path::new(path).join(file_path_from_folder);
        let destiny_parent_path = destiny_path.parent().unwrap();
        create_dir_all(destiny_parent_path).expect("error creating the path");
        let _ =
            File::create(&destiny_path).expect("Should have been able to read the destiny path");
    }
    fn get_file_path_from_folder<'a>(&'a self, file: &'a Path) -> &Path {
        let parent_path = Path::new(&self.name)
            .parent()
            .expect("doestn have a parent");
        let file_whithout_path = file
            .strip_prefix(parent_path)
            .expect("is not a prefix of the file");
        file_whithout_path
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
    let mut folder = Folder::from_path(source);
    folder.load_files_from_path();
    let start = Instant::now();
    folder.copy_to(destiny);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

// fn copy(source_path: &str, destiny_path: &str, capacity: usize) {
//     let destiny_file =
//         File::create(destiny_path).expect("Should have been able to read the destiny path");
//     let source_file =
//         File::open(source_path).expect("Should have been able to read the source path");

//     let mut stream = BufWriter::with_capacity(capacity, &destiny_file);
//     let mut reader = BufReader::with_capacity(capacity, &source_file);

//     let size = source_file.metadata().unwrap().len() as usize;
//     let mut progress = progress_bar::ProgressBar::from_total_size(size);
//     println!();
//     loop {
//         let buffer = reader.fill_buf().expect("error in the buffer");
//         let lenght = buffer.len();
//         stream.write_all(buffer).expect("error to write");
//         reader.consume(lenght);
//         progress.consume(lenght);
//         if lenght == 0 {
//             break;
//         }
//     }
//     println!()
// }
