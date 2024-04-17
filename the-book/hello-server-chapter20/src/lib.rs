use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
use std::mem::drop;



type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    // the presence of the sender indicates that ThreadPool is active
    sender: Option< mpsc::Sender<Job> >,
}

impl ThreadPool {
    /// Create a new ThreadPool. If the number of threads requested (size) is 0 , it will panic.
    /// # Panics
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // we use a mpsc channel and we then distribute the tx by giving a shared reference to it (mutex)
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down.");
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option< thread::JoinHandle<()> >,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("Something went wrong in the worker {id}, the mutex lock returned an error")
                .recv();

            match message {
                Ok(job) => {
                    println!("\n&&&&&&&&&&&&&&\nWorker {id} got a job; executing.\n&&&&&&&&&&&&&&\n");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        Worker { id, thread: Some(thread) }
    }
}
