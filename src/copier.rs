use crate::{
    copier_pool::CopierPool,
    progress_counter::{ProgressBar, ProgressCounter},
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};
//TODO replace with a tuple with a type alias
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
}
impl Copier {
    pub fn from_folder_to_dir() -> Self {
        Self { paused: false }
    }
    pub fn start(&mut self, files: impl Iterator<Item = FileCopy>) {
        let pool = CopierPool::new(4);

        for FileCopy {
            source_file,
            target_file,
        } in files
        {
            if !self.paused {
                let file_size = source_file.metadata().unwrap().len() as usize;
                pool.execute(
                    move |bar: &mut ProgressBar| {
                        create_file(bar, &source_file, &target_file).expect("error copy the file");
                    },
                    file_size,
                );
                self.paused = false;
            }
        }
    }
}
fn create_file<T: ProgressCounter>(
    progress_bar: &mut T,
    source_file: &PathBuf,
    target_file: &PathBuf,
) -> Result<(), io::Error> {
    let destiny_file = File::open(source_file).expect("error opening the file");
    let to_copy_file = File::create(target_file).expect("error creating the file");
    progress_bar.set_new_file(source_file);

    copy_file(progress_bar, destiny_file, to_copy_file, 1024 * 500);
    Ok(())
}
fn copy_file<T: ProgressCounter>(
    progress_bar: &mut T,
    source_file: File,
    destiny_file: File,
    capacity: usize,
) {
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
        progress_bar.consume(buffer_lenght);
    }
}
