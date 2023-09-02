use crate::{copier_pool::CopierPool, progress_counter::ProgressBar};

use super::FileCopy;

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
