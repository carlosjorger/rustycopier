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

use crate::progress_counter::ProgressBar;
// TODO: use ProgressCounter trait in copier_pool
pub struct CopierPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
type WorkerJob = Box<dyn FnOnce(&mut ProgressBar) + Send + 'static>;
type Job = (WorkerJob, usize);
impl CopierPool {
    pub fn new(worker_number: usize) -> Self {
        assert!(worker_number > 0);
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(worker_number);
        let receiver = Arc::new(Mutex::new(receiver));
        let shared_stdout = Arc::new(Mutex::new(stdout()));
        clean_terminal();
        for id in 0..worker_number {
            workers.push(Worker::new(
                id,
                worker_number,
                Arc::clone(&receiver),
                Arc::clone(&shared_stdout),
            ));
        }

        CopierPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F, file_size: usize)
    where
        F: FnOnce(&mut ProgressBar) + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = self.sender.as_ref() {
            sender.send((job, file_size)).unwrap();
        }
    }
}
impl Drop for CopierPool {
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
        receiver: Arc<Mutex<Receiver<Job>>>,
        shared_stdout: Arc<Mutex<Stdout>>,
    ) -> Worker {
        let mut progress_bar = ProgressBar::new(id as u16, total_of_workers as u16, shared_stdout);
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
fn execute_jobs_queue(job_queue: &mut LinkedList<WorkerJob>, progress_bar: &mut ProgressBar) {
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
