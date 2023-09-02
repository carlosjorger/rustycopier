use crate::{
    copier_pool::CopierPool,
    progress_counter::{ProgressBar, ProgressCounter},
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};
// TODO: create separete module for this type
pub struct FileCopy {
    source_file_path: PathBuf,
    source_file: File,
    target_file: File,
}
impl FileCopy {
    pub fn from_files(target_file_path: PathBuf, source_file_path: PathBuf) -> Self {
        let source_file: File = File::open(&source_file_path).expect("error opening the file");
        let target_file = File::create(target_file_path).expect("error creating the file");
        Self {
            source_file_path,
            source_file,
            target_file,
        }
    }
    fn create_file<T: ProgressCounter>(&mut self, progress_bar: &mut T) -> Result<(), io::Error> {
        progress_bar.set_new_file(&self.source_file_path);

        self.copy_file(progress_bar, 1024 * 500);
        Ok(())
    }
    fn copy_file<T: ProgressCounter>(&self, progress_bar: &mut T, capacity: usize) {
        let mut stream = BufWriter::with_capacity(capacity, &self.target_file);
        let mut reader = BufReader::with_capacity(capacity, &self.source_file);

        loop {
            let buffer = reader.fill_buf().expect("error in the buffer");
            let buffer_lenght = buffer.len();
            if buffer_lenght == 0 {
                break;
            }
            stream.write_all(buffer).expect("error to write");
            reader.consume(buffer_lenght);
            progress_bar.consume(buffer_lenght);
        }
    }
}
pub struct Copier {
    paused: bool,
}
impl Copier {
    pub fn from_folder_to_dir() -> Self {
        Self { paused: false }
    }
    pub fn start(&mut self, files: impl Iterator<Item = FileCopy>) {
        let pool = CopierPool::new(4);

        for mut file_copy in files {
            if !self.paused {
                let file_size = file_copy.source_file_path.metadata().unwrap().len() as usize;
                pool.execute(
                    move |bar: &mut ProgressBar| {
                        file_copy.create_file(bar).expect("error copy the file");
                    },
                    file_size,
                );
                self.paused = false;
            }
        }
    }
}
