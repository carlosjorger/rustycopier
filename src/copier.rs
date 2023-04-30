use crate::progress_bar;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};
//TODO choose a better type for files
pub struct FileCopy {
    source_file: PathBuf,
    target_file: PathBuf,
}
impl FileCopy {
    pub fn from_files(to_file: PathBuf, file: PathBuf) -> Self {
        Self {
            source_file: file,
            target_file: to_file,
        }
    }
}
pub struct Copier {
    paused: bool,
    progress_bar: progress_bar::ProgressBar,
}
impl Copier {
    pub fn from_folder_to_dir(total_size: usize) -> Self {
        Self {
            paused: false,
            progress_bar: progress_bar::ProgressBar::from_total_size(total_size),
        }
    }
    pub fn start(&mut self, files: impl Iterator<Item = FileCopy>) {
        for file in files {
            if !self.paused {
                self.create_file(&file).expect("error copy the file");
                self.paused = false;
            }
        }
    }
    fn create_file(&mut self, file_to_copy: &FileCopy) -> Result<(), io::Error> {
        let destiny_file = File::open(&file_to_copy.source_file).expect("error opening the file");
        let to_copy_file =
            File::create(&file_to_copy.target_file).expect("error creating the file");

        if let Some(file_name) = file_to_copy.target_file.file_name() {
            self.progress_bar.set_new_file(file_name.to_str().unwrap());
        }

        self.copy_file(destiny_file, to_copy_file, 1024 * 500);
        Ok(())
    }
    fn copy_file(&mut self, source_file: File, destiny_file: File, capacity: usize) {
        let mut stream = BufWriter::with_capacity(capacity, &destiny_file);
        let mut reader = BufReader::with_capacity(capacity, &source_file);

        loop {
            let buffer = reader.fill_buf().expect("error in the buffer");
            let lenght = buffer.len();
            stream.write_all(buffer).expect("error to write");
            reader.consume(lenght);
            self.progress_bar.consume(lenght);
            if lenght == 0 {
                break;
            }
        }
    }
}
