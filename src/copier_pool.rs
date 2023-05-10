use std::{
    collections::LinkedList,
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread,
};

use crossterm::queue;

use crate::progress_bar::ProgressBar;

pub struct CopierPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
type WorkerJob = Box<dyn FnOnce(&mut ProgressBar) + Send + 'static>;
type Job = (WorkerJob, usize);
impl CopierPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
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
    id: usize,
    load_jobs_thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let mut progress_bar = ProgressBar::new(id as u16);
        let mut job_queue: LinkedList<WorkerJob> = LinkedList::new();
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok((job, file_size)) => {
                    progress_bar.add_size(file_size);
                    // job(&mut progress_bar);
                    job_queue.push_back(job);
                }
                Err(_) => {
                    execute_jobs_queue(&mut job_queue, &mut progress_bar);
                    break;
                }
            }
        });
        // thread::spawn(move || loop {
        //     while let job = job_queue.pop_back() {}
        // });
        Worker {
            id,
            load_jobs_thread: Some(thread),
        }
    }
}
fn execute_jobs_queue(job_queue: &mut LinkedList<WorkerJob>, progress_bar: &mut ProgressBar) {
    while let Some(job) = job_queue.pop_back() {
        job(progress_bar);
    }
}
