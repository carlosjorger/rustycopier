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
            progress_bar: progress_bar::ProgressBar::new(total_size),
        }
    }
    pub fn start(&mut self, files: impl Iterator<Item = FileCopy>) {
        for FileCopy {
            source_file,
            target_file,
        } in files
        {
            if !self.paused {
                self.create_file(&source_file, &target_file)
                    .expect("error copy the file");
                self.paused = false;
            }
        }
    }
    fn create_file(
        &mut self,
        source_file: &PathBuf,
        target_file: &PathBuf,
    ) -> Result<(), io::Error> {
        let destiny_file = File::open(source_file).expect("error opening the file");
        let to_copy_file = File::create(target_file).expect("error creating the file");

        if let Some(file_name) = target_file.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                self.progress_bar.set_new_file(file_name_str);
            }
        }

        self.copy_file(destiny_file, to_copy_file, 1024 * 500);
        Ok(())
    }
    fn copy_file(&mut self, source_file: File, destiny_file: File, capacity: usize) {
        let mut stream = BufWriter::with_capacity(capacity, &destiny_file);
        let mut reader = BufReader::with_capacity(capacity, &source_file);

        loop {
            let buffer = reader.fill_buf().expect("error in the buffer");
            let buffer_lenght = buffer.len();
            if buffer_lenght == 0 {
                break;
            }
            stream.write_all(buffer).expect("error to write");
            reader.consume(buffer_lenght);
            self.progress_bar.consume(buffer_lenght);
        }
    }
}
