use std::{
    collections::LinkedList,
    io::Stdout,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

use crate::progress_counter::{CustomProgressBar, ProgressCounter};

use super::{Job, WorkerJob};

pub struct Worker {
    pub load_jobs_thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(
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
pub fn execute_jobs_queue<T: ProgressCounter>(
    job_queue: &mut LinkedList<WorkerJob<T>>,
    progress_bar: &mut T,
) {
    while let Some(job) = job_queue.pop_back() {
        job(progress_bar);
    }
}
