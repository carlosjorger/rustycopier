use std::{
    io::{stdout, Write},
    sync::{
        mpsc::{self},
        Arc, Mutex,
    },
};

use crate::progress_counter::{CustomProgressBar, ProgressCounter};
use crossterm::{
    cursor::{self},
    terminal::size,
    QueueableCommand,
};

use self::worker::Worker;
pub mod worker;

pub struct CopierPool<T: ProgressCounter> {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job<T>>>,
}
type WorkerJob<T> = Box<dyn FnOnce(&mut T) + Send + 'static>;
type Job<T> = (WorkerJob<T>, usize);
// TODO: try to implement rown robin
impl CopierPool<CustomProgressBar> {
    pub fn new(worker_number: usize, is_logging_active: bool) -> Self {
        assert!(worker_number > 0);
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(worker_number);
        let receiver = Arc::new(Mutex::new(receiver));
        let shared_stdout = Arc::new(Mutex::new(stdout()));
        if is_logging_active {
            clean_terminal();
        }
        for id in 0..worker_number {
            workers.push(Worker::new(
                id,
                worker_number,
                Arc::clone(&receiver),
                Arc::clone(&shared_stdout),
                is_logging_active,
            ));
        }

        CopierPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F, file_size: usize)
    where
        F: FnOnce(&mut CustomProgressBar) + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = self.sender.as_ref() {
            sender.send((job, file_size)).unwrap();
        }
    }
}
impl<T: ProgressCounter> Drop for CopierPool<T> {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            if let Some(thread) = worker.load_jobs_thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
fn clean_terminal() {
    let (_, terminal_length) = size().unwrap();
    println!("Copy in progress....");
    println!();
    for _ in 0..(terminal_length - 3) {
        println!();
    }
    let mut std = stdout().lock();
    std.queue(cursor::MoveToRow(2)).unwrap();
    std.flush().unwrap();
}
