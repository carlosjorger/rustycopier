use crossterm::{cursor, terminal, QueueableCommand};
use std::{
    io::{stdout, Stdout, Write},
    path::PathBuf,
};

struct ProgressBarDrawer {
    stdout: Stdout,
    total_number_of_bars: usize,
    progress_window: String,
    rest_window: String,
    stdout_position: u16,
}
impl ProgressBarDrawer {
    fn progress_bar(total_number_of_bars: usize, stdout_position: u16) -> Self {
        let progress_window = "=".repeat(total_number_of_bars);
        let rest_window = "-".repeat(total_number_of_bars);
        Self {
            stdout: stdout(),
            total_number_of_bars,
            progress_window,
            rest_window,
            stdout_position,
        }
    }
    fn draw_a_bar(&mut self, number_of_bars: usize) {
        self.move_to_line(self.stdout_position);
        print!(
            "\r Copying: [{}{}] {}%",
            &self.progress_window[0..number_of_bars],
            &self.rest_window[0..(self.total_number_of_bars - number_of_bars)],
            self.percentage_of_number_of_bars(number_of_bars),
        );
        self.move_to_line(self.stdout_position + 1);
        print!("------------------------------------------");
        self.stdout.flush().unwrap();
    }
    fn percentage_of_number_of_bars(&self, number_of_bars: usize) -> f64 {
        (number_of_bars as f64 * (100.0 / (self.total_number_of_bars as f64))).trunc()
    }
    fn print_new_file(&mut self, file_name: &str) {
        self.move_to_line(self.stdout_position);
        self.clean_from_cursor_down();
        self.move_to_line(2 + self.stdout_position);
        print!("{file_name}");
        self.stdout.flush().unwrap();
    }
    fn clean_from_cursor_down(&mut self) {
        self.stdout
            .queue(cursor::MoveToRow(self.stdout_position + 2))
            .unwrap()
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
    }
    fn move_to_line(&mut self, stdout_position: u16) {
        self.stdout
            .queue(cursor::MoveToRow(stdout_position))
            .unwrap()
            .queue(cursor::MoveToColumn(0))
            .unwrap();
    }
}
pub struct ProgressBar {
    total_size: usize,
    consumed_size: usize,
    progress_bar: ProgressBarDrawer,
    total_of_bars: usize,
    finished: bool,
}
impl ProgressBar {
    pub fn new(position: u16) -> Self {
        const NUMBER_OF_BARS: usize = 25;
        Self {
            total_size: 0,
            consumed_size: 0,
            progress_bar: ProgressBarDrawer::progress_bar(NUMBER_OF_BARS, position * 4 + 1),
            total_of_bars: NUMBER_OF_BARS,
            finished: false,
        }
    }
    pub fn set_new_file(&mut self, file_path: &PathBuf) {
        if let Some(file_name) = file_path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                self.progress_bar.print_new_file(file_name_str);
            }
        }
    }
    pub fn add_size(&mut self, size: usize) {
        self.total_size += size;
    }
    pub fn consume(&mut self, lenght: usize) {
        self.consumed_size += lenght;
        let approximate_number_of_bars: usize =
            (self.fraction_of_consume() * (self.total_of_bars as f64)).round() as usize;
        if !self.finished {
            self.progress_bar.draw_a_bar(approximate_number_of_bars);
        }
        self.finished = approximate_number_of_bars == self.total_of_bars;
    }
    fn fraction_of_consume(&self) -> f64 {
        (self.consumed_size as f64) / (self.total_size as f64)
    }
}
