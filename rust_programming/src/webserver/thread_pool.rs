pub mod thread_pool {
    use std::{
        sync::{mpsc, Arc, Mutex},
        thread,
    };

    pub struct ThreadPool {
        workers: Vec<Worker>,

        // The ThreadPoll will create a channel and hold on to the sender.
        // Each worker will hold on to the receiver.
        // The Job struct will hold the closures we want to send down the channel.
        // The execute method will send the job it wants to execute through the sender.
        sender: Option<mpsc::Sender<Job>>,
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    impl ThreadPool {
        // create a new ThreadPool.
        // The size is the number of threads in the pool.
        //
        // # Panics
        //
        // The `new` function will panic if the size is less than 1.
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);

            let (sender, receiver) = mpsc::channel();

            // we put the receiver in an Arc and a Mutex, for each new worker, we clone the Arc to bump
            // the reference count so the workers can share ownership of the receiver.
            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            // We want to create the threads and have them wait for code that we'll send later.
            for id in 0..size {
                // create some threads and store then in the vector.
                workers.push(Worker::new(id, receiver.clone()));
            }

            ThreadPool {
                workers,
                sender: Some(sender),
            }
        }

        // FnOnce trait bound is used since the closure will only be run once.
        // Send trait bound is used to transfer the closure from one thread to another
        // 'static trait bound is used since we don't know how long the thread will take to finish execution.
        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            // create a new Job instance using the closure we get in execute.
            let job = Box::new(f);

            // send the job using the sending end of the channel.
            self.sender.as_ref().unwrap().send(job).unwrap();
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            // Dropping sender closes the channel, which indicates no more messages willbe sent.
            // When that happens, all the calls to recv that the workers do in the infinite loop
            // will return an error, which means that the threads will finish when the ThreadPool
            // drop implementation calls join on them.
            drop(self.sender.take());

            for worker in &mut self.workers {
                println!("Shutting down worker {}", worker.id);

                // we call the take method on the Option to move the value out of the Some variant
                // and leave a None variant in its place.
                // - i.e a worker that is running will have a Some variant in thread, and when we
                // want to clean up a  worker, we'll replace Some with None so the worker doesn't
                // have a thread to run.
                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }

    // The worker thread will create threads and wait for the code.
    // Instead of storing a vector of JoinHandle<()> instances in the thread pool, we'll store
    // instances of the worker struct.
    // - Each worker will store a single JoinHandle<()> instance.
    // - Then we'll implement a method on Woker that will take a closure of code to run and and send it
    // to the already running thread for execution.
    // - Each worker will have an id so that we can distinguish the different workers in the pool when
    // logging and debugging.
    struct Worker {
        id: usize,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            // NOTE: if the operating system can't create a thread because there aren't enough system
            // resources, thread::spawn will panic. That will cause the server to panic. To mitigate
            // this, use std::thread::Builder::spawn instead, it returns a Result type.
            //
            // we need the closure to loop forever, asking the receiving end of the channle for a job
            // and running the job when it gets one.
            let thread = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                };
            });

            return Worker {
                id,
                thread: Some(thread),
            };
        }
    }
}
