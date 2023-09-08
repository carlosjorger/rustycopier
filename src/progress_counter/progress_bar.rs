use crossterm::{
    cursor::{self, position},
    terminal, QueueableCommand,
};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressState, ProgressStyle};
use std::{
    fmt::Write,
    io::{Stdout, Write as ioWrite},
    path::Path,
    sync::{Arc, Mutex, MutexGuard},
};

use super::ProgressCounter;

pub struct CustomProgressBar {
    total_size: usize,
    consumed_size: usize,
    progress_bar: ProgressBarDrawer,
    finished: bool,
    is_logging_active: bool,
}
impl ProgressCounter for CustomProgressBar {
    fn set_new_file(&mut self, file_path: &Path) {
        if !self.is_logging_active {
            return;
        }
        if let Some(file_name) = file_path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                self.progress_bar.print_new_file(file_name_str);
            }
        }
    }
    fn consume(&mut self, lenght: usize) {
        self.consumed_size += lenght;
        if !self.finished && self.is_logging_active {
            self.progress_bar.draw_a_bar(self.consumed_size);
        }
        self.finished = self.consumed_size == self.total_size;
    }
    fn add_size(&mut self, size: usize) {
        self.total_size += size;
        self.progress_bar.bar.set_length(self.total_size as u64);
    }
}
impl CustomProgressBar {
    pub fn new(
        progress_bar_position: u16,
        total_of_progress_bar: u16,
        stout_mutex: Arc<Mutex<Stdout>>,
        is_logging_active: bool,
    ) -> Self {
        let (_, stdout_position) = position().unwrap();
        Self {
            total_size: 0,
            consumed_size: 0,
            progress_bar: ProgressBarDrawer::progress_bar(
                0,
                stdout_position + progress_bar_position * 2,
                stdout_position + total_of_progress_bar * 2,
                stout_mutex,
            ),
            finished: false,
            is_logging_active,
        }
    }
}
struct ProgressBarDrawer {
    stdout: Arc<Mutex<Stdout>>,
    stdout_position: u16,
    final_stdout_position: u16,
    bar: ProgressBar,
}
impl ProgressBarDrawer {
    fn progress_bar(
        total_size_of_bar: usize,
        stdout_position: u16,
        final_stdout_position: u16,
        stout_mutex: Arc<Mutex<Stdout>>,
    ) -> Self {
        let bar = ProgressBar::new(total_size_of_bar as u64);
        bar.set_draw_target(ProgressDrawTarget::stdout());
        // TODO: Set the file at the end of the progress bar https://github.com/console-rs/indicatif/blob/main/examples/multi.rs
        bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
        Self {
            stdout: stout_mutex,
            stdout_position,
            final_stdout_position,
            bar,
        }
    }
    fn draw_a_bar(&mut self, number_of_bars: usize) {
        let mut stdout_result = self.stdout.lock().unwrap();
        self.move_to_line(self.stdout_position, &mut stdout_result);
        self.bar.set_position(number_of_bars as u64);
        self.move_to_line(self.final_stdout_position, &mut stdout_result);
        stdout_result.flush().unwrap();
    }

    fn print_new_file(&self, file_name: &str) {
        let mut stdout_result = self.stdout.lock().unwrap();
        self.move_line_and_clean(self.stdout_position + 1, &mut stdout_result);
        print!("Copying {file_name}...");
        stdout_result.flush().unwrap();
    }
    fn move_line_and_clean(&self, position: u16, stdout: &mut MutexGuard<'_, Stdout>) {
        self.move_to_line(position, stdout);
        self.clean_line(position, stdout);
    }
    fn clean_line(&self, position: u16, stdout: &mut MutexGuard<'_, Stdout>) {
        stdout
            .queue(cursor::MoveToRow(position))
            .unwrap()
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
    }
    fn move_to_line(&self, stdout_position: u16, stdout: &mut MutexGuard<'_, Stdout>) {
        stdout.queue(cursor::MoveTo(0, stdout_position)).unwrap();
    }
}
impl Drop for ProgressBarDrawer {
    fn drop(&mut self) {
        let mut stdout_result = self.stdout.lock().unwrap();
        self.move_to_line(self.final_stdout_position, &mut stdout_result);
    }
}
