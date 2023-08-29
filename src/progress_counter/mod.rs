pub mod progress_bar;

use std::path::Path;

pub trait ProgressCounter {
    fn set_new_file(&mut self, file_path: &Path);
    fn consume(&mut self, lenght: usize);
}
