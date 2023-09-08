use std::{
    collections::LinkedList,
    io::{stdout, Stdout, Write},
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread,
};

use crossterm::{
    cursor::{self},
    terminal::size,
    QueueableCommand,
};

use crate::progress_counter::{CustomProgressBar, ProgressCounter};
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
struct Worker {
    load_jobs_thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        total_of_workers: usize,
        receiver: Arc<Mutex<Receiver<Job<CustomProgressBar>>>>,
        shared_stdout: Arc<Mutex<Stdout>>,
        is_logging_active: bool,
    ) -> Worker {
        let mut progress_bar = CustomProgressBar::new(
            id as u16,
            total_of_workers as u16,
            shared_stdout,
            is_logging_active,
        );
        let mut job_queue = LinkedList::new();
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok((job, file_size)) => {
                    progress_bar.add_size(file_size);
                    job_queue.push_back(job);
                }
                Err(_) => {
                    execute_jobs_queue(&mut job_queue, &mut progress_bar);
                    break;
                }
            }
        });
        Worker {
            load_jobs_thread: Some(thread),
        }
    }
}
fn execute_jobs_queue<T: ProgressCounter>(
    job_queue: &mut LinkedList<WorkerJob<T>>,
    progress_bar: &mut T,
) {
    while let Some(job) = job_queue.pop_back() {
        job(progress_bar);
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
