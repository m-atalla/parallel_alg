//! # The Thread Pool Implementation
//! The [ThreadPool] struct, contains the following data: 
//! - List of the worker threads' handles
//! - Channel sender object
//!
//! The thread pool is reponsible for dispatching jobs to the channel via the sender object
//! to allow thread workers to execute it.
//!
//! # Worker Thread
//! the [Worker] thread algorithm is pretty simple, it does the following:
//! 1. waits for a new job to be received 
//! 1. acquire the lock for the received job
//! 1. execute the locked job
//! 1. repeat from (1)
//!
//! this goes on until the thread pool gets deallocated check [ThreadPool::drop] for more on how
//! this is done gracefully.
//!
//! # Important Note
//! this implementation of a thread pool is similar to the one discussed in the rust [book](https://doc.rust-lang.org/book/)
//! and I find it to be simpler to explain and solves the class problems efficiently so far.
//!
//! There are other rust libraries (*crates*) that does pretty much the same thing but they're alot more
//! complex and introduce extra complexity into the code. However, they're still worth considering
//! depending on the task at hand.
//!
//! One cool alternative I found was [Rayon](https://crates.io/crates/rayon) that enables automatic
//! parallelization and uses the [work stealing](https://en.wikipedia.org/wiki/Work_stealing)
//! algorithm.
//!
use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;


impl ThreadPool {
    /// Creates a new thread pool object by doing the following:
    ///
    /// - creates a list of worker threads
    /// - creates the communication channel
    /// - spawns the worker threads and appends them to the list
    ///
    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        // Atomic reference counted pointer
        // since this is a muliple produce/single consumer channel
        // a mutex is used to allow for this receiver object to be safely
        // acquired by a single thread without fear of a race condition occuring.
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(
                Worker::new(id, Arc::clone(&receiver))
            );
        }

        ThreadPool { 
            workers,
            sender: Some(sender)
        }
    }

    /// allocates the given closure on the heap via a [Box](https://doc.rust-lang.org/std/boxed/index.html)
    /// and sends the pointer to that closure in the channel to be executed by a worker
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    pub fn size(&self) -> usize {
        self.workers.len()
    }
}

/// Gracefully drops the thread pool
/// by making each worker thread *join* the main thread
/// while allowing it to finish the currently executing job (if any exists).
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[allow(dead_code)]
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// 
    /// Spawns the worker thread with a loop that waits for a job to be received 
    /// and then locked for execution.
    ///
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => { 
                        job(); 
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        });

        Worker { 
            id,
            thread: Some(thread)
        }
    }
}
