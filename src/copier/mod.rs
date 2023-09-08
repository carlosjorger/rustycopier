pub mod file_copy;
pub mod file_to_copy;
pub use file_copy::FileCopy;
pub use file_to_copy::FileToCopy;
#[cfg(test)]
mod file_to_copy_test;
use crate::{copier_pool::CopierPool, progress_counter::CustomProgressBar};
pub struct Copier {
    paused: bool,
}
impl Copier {
    pub fn from_folder_to_dir() -> Self {
        Self { paused: false }
    }
    pub fn start(&mut self, files: impl Iterator<Item = FileCopy>, is_logging_active: bool) {
        let pool = CopierPool::new(5, is_logging_active);

        for mut file_copy in files {
            if !self.paused {
                let file_size = file_copy.source_file_path.metadata().unwrap().len() as usize;
                pool.execute(
                    move |bar: &mut CustomProgressBar| {
                        file_copy.create_file(bar).expect("error copy the file");
                    },
                    file_size,
                );
                self.paused = false;
            }
        }
    }
}
