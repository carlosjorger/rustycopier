use crate::progress_bar;
use std::{
    fs::{create_dir_all, File},
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

pub struct FileToCopy {
    file: PathBuf,
    to_file: PathBuf,
}
impl FileToCopy {
    pub fn from_files(to_file: PathBuf, file: PathBuf) -> Self {
        Self { file, to_file }
    }
}
pub struct Copier<'a> {
    files: &'a Vec<FileToCopy>,
    paused: bool,
}
impl<'a> Copier<'a> {
    pub fn from_folder_to_dir(files: &'a Vec<FileToCopy>) -> Self {
        Self {
            files,
            paused: false,
        }
    }
    pub fn copy(&mut self) {
        for file in self.files {
            if !self.paused {
                self.create_file(file).expect("error copy the file");
                self.paused = false;
            }
        }
    }
    fn create_file(&self, file_to_copy: &FileToCopy) -> Result<(), io::Error> {
        let file = &file_to_copy.to_file;
        let file_folder = file.parent().expect("doestn have a parent");
        create_dir_all(file_folder).expect("error creating the folder");
        println!(
            "<{},{},{}>",
            file_folder.to_str().unwrap(),
            file_to_copy.file.to_str().unwrap(),
            file_to_copy.to_file.to_str().unwrap()
        );
        let destiny_file = File::open(&file_to_copy.file).expect("error opening the file");
        let to_copy_file = File::create(&file_to_copy.to_file).expect("error creating the file");

        self.copy_file(destiny_file, to_copy_file, 1024);
        Ok(())
    }
    fn copy_file(&self, source_file: File, destiny_file: File, capacity: usize) {
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
}
