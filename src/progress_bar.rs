use crossterm::{cursor, terminal, QueueableCommand};
use std::io::{stdout, Stdout, Write};

struct ProgressBarDrawer {
    stdout: Stdout,
    scale: usize,
}
impl ProgressBarDrawer {
    fn progress_bar(scale: usize) -> Self {
        Self {
            stdout: stdout(),
            scale,
        }
    }
    fn draw_a_bar(&mut self, progress_base_ten: usize) {
        let column_position = (progress_base_ten) as u16;
        self.stdout
            .queue(cursor::MoveToColumn(column_position))
            .unwrap();
        if progress_base_ten == self.scale {
            print!("=");
        } else {
            print!("=>");
        }
    }
    fn change_progress_number(&mut self, fraction_of_consume: f64) {
        self.stdout
            .queue(cursor::MoveToColumn((self.scale + 2) as u16))
            .unwrap()
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
            .unwrap();
        let percent_of_consume = (fraction_of_consume as f32) * 100.0;
        if fraction_of_consume == 1.0 {
            print!("{}/100", format_args!("{:}", percent_of_consume));
        } else {
            print!("{}/100", format_args!("{:.2}", percent_of_consume));
        }

        self.stdout.flush().unwrap();
    }
}
pub struct ProgressBar {
    total_size: usize,
    consumed_size: usize,
    approximate_progres: usize,
    progress_bar: ProgressBarDrawer,
    number_of_bars: usize,
}
impl ProgressBar {
    pub fn from_total_size(total_size: usize) -> Self {
        const NUMBER_OF_BARS: usize = 25;
        Self {
            total_size,
            consumed_size: 0,
            approximate_progres: 0,
            progress_bar: ProgressBarDrawer::progress_bar(NUMBER_OF_BARS),
            number_of_bars: NUMBER_OF_BARS,
        }
    }
    fn percent_of_consume(&self) -> f64 {
        (self.consumed_size as f64) / (self.total_size as f64)
    }
    fn percent_of_progress(&self) -> f64 {
        (self.approximate_progres as f64) / (self.number_of_bars as f64)
    }
    pub fn consume(&mut self, lenght: usize) {
        self.consumed_size += lenght;
        if self.percent_of_consume() > self.percent_of_progress() {
            self.approximate_progres += 1;
            self.progress_bar.draw_a_bar(self.approximate_progres);
        }
        self.progress_bar
            .change_progress_number(self.percent_of_consume())
    }
}
