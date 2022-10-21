use std::{
    sync::{
        mpsc::{self},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

// A "Job" is just a closure function that is run when
// executing on the thread pool. It receives its thread id for
// reference.
// so use it like this:
// thread_pool.execute(|id| { print!("Hi, I'm Thread {id}."});
type Job = Box<dyn FnOnce(usize) + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let mut w = Worker { id, thread: None };
        w.start(receiver);

        return w;
    }

    fn start(&mut self, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) {
        let id = self.id;
        self.thread = Some(thread::spawn(move || loop {
            // lock the mutex before receiving a msg, to avoid mutual access:
            let guard = receiver.lock().unwrap();
            let msg = guard.recv(); // implements deref, so access to the inner channel receiver can be done here
                                    // drop the mutex guard early, otherwise it would block
                                    // the lock the whole time
            drop(guard);
            match msg {
                Ok(job) => {
                    job(id);
                }
                Err(_) => {
                    println!("Thread {}: Disconnected, shutting down...", id);
                    break;
                }
            }
        }));
    }

    fn join(&mut self) {
        // Why that complicated? thread.join() does take
        // ownership of thread: but it cannot be moved out of
        // the Worker. So we wrap it in an Option, to be
        // able to move the ownership out of the Option (Some):
        // take() takes the value out of the Option and returns
        // ownership:
        if let Some(t) = self.thread.take() {
            t.join().unwrap();
        }
    }
}

pub struct ThreadPool {
    nr_of_threads: usize,
    sender: Option<mpsc::Sender<Job>>,
    workers: Vec<Option<Worker>>,
}

impl ThreadPool {
    pub fn builder(nr_of_threads: usize) -> ThreadPool {
        // used to send new jobs to the workers: the receiver
        // is shared in the workers.
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(nr_of_threads);

        for id in 1..=nr_of_threads {
            let w = Worker::new(id, receiver.clone());
            workers.push(Some(w));
        }

        ThreadPool {
            nr_of_threads: match nr_of_threads {
                0 => 1,
                _ => nr_of_threads,
            },
            sender: Some(sender),
            workers,
        }
    }

    pub fn execute<T>(&self, f: T)
    where
        T: FnOnce(usize) + Send + 'static,
    {
        // send new job by using the channel:
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }

    pub fn shutdown(&mut self) {
        // signalling the workers to shhut down by
        // closing the channel's sender:
        drop(self.sender.take());
        // then wait for all workers to be done:
        for w in &mut self.workers {
            w.take().unwrap().join();
        }
    }
}
