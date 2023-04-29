use crossterm::{cursor, terminal, QueueableCommand};
use std::{
    cmp::min,
    io::{stdout, Stdout, Write},
};

struct ProgressBarDrawer {
    stdout: Stdout,
    scale: usize,
    progress_window: String,
    rest_window: String,
}
impl ProgressBarDrawer {
    fn progress_bar(scale: usize) -> Self {
        let progress_window = "=".repeat(scale);
        let rest_window = "-".repeat(scale);
        Self {
            stdout: stdout(),
            scale,
            progress_window,
            rest_window,
        }
    }
    fn draw_a_bar(&mut self, progress_base_ten: usize) {
        print!(
            "\r Copying: [{}{}] {}%",
            &self.progress_window[0..progress_base_ten],
            &self.rest_window[0..(self.scale - progress_base_ten)],
            (progress_base_ten as f64 * (100.0 / (self.scale as f64))).trunc()
        );
        self.stdout.flush().unwrap();
    }
    fn print_new_file(&mut self, file_name: &str) {
        println!();
        self.clean_from_cursor_down();
        print!("{file_name}");
        self.stdout.queue(cursor::MoveToPreviousLine(1)).unwrap();
        self.stdout.flush().unwrap();
    }
    fn clean_from_cursor_down(&mut self) {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
            .unwrap();
    }
}
pub struct ProgressBar {
    total_size: usize,
    consumed_size: usize,
    approximate_progres: usize,
    progress_bar: ProgressBarDrawer,
    total_of_bars: usize,
}
impl ProgressBar {
    pub fn from_total_size(total_size: usize) -> Self {
        const NUMBER_OF_BARS: usize = 25;
        Self {
            total_size,
            consumed_size: 0,
            approximate_progres: 0,
            progress_bar: ProgressBarDrawer::progress_bar(NUMBER_OF_BARS),
            total_of_bars: NUMBER_OF_BARS,
        }
    }

    pub fn set_new_file(&mut self, file_name: &str) {
        self.progress_bar.print_new_file(file_name);
    }
    pub fn consume(&mut self, lenght: usize) {
        self.consumed_size += lenght;
        self.approximate_progres = min(
            (self.percent_of_consume() * (self.total_of_bars as f64).round()) as usize,
            self.total_of_bars,
        );
        self.progress_bar.draw_a_bar(self.approximate_progres);
    }
    fn percent_of_consume(&self) -> f64 {
        (self.consumed_size as f64) / (self.total_size as f64)
    }
}
