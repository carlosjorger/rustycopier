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
    sender: Option<mpsc::Sender<WorkerJob>>,
}
type Job = (Box<dyn FnOnce(&mut ProgressBar) + Send + 'static>, usize);

type WorkerJob = Box<dyn FnOnce(&mut ProgressBar) + Send + 'static>;
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
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(&mut ProgressBar) + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = self.sender.as_ref() {
            sender.send(job).unwrap();
        }
    }
}
impl Drop for CopierPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<WorkerJob>>>) -> Worker {
        let mut progress_bar = ProgressBar::new(id as u16);
        // let mut queue: LinkedList<Box<Job>> = LinkedList::new();
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => job(&mut progress_bar),
                Err(_) => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
