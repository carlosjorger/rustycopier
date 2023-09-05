pub mod progress_bar;
pub use progress_bar::ProgressBar;
use std::path::Path;

pub trait ProgressCounter {
    fn set_new_file(&mut self, file_path: &Path);
    fn consume(&mut self, lenght: usize);
    fn add_size(&mut self, size: usize);
}
