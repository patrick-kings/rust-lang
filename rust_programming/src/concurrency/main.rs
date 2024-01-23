fn main() {
    println!("fearless concurrency");

    concurrency::run();
}
// Fearless concurrency
//
//  - How to create threads to run multiple pieces of code ata the same time
//  - Message-passing concurrency, where channels send messages between threads
//  - Shared-state concurrency, where multiple threads have access to some piece of data
//  - The `Sync` and `Send` traits, which extend Rust's concurrency guarantees to user-defined
//    types as well as types provided by the standard library.
//

// Using threads to run code simultaneously
//
// The Rust standard library uses a 1:1 model of thread implementation, wherby a program uses one
// operating system thread per one language thread. However, there are crates that implement other
// threading models that make different tradeoffs to the 1:1 model
pub mod concurrency {
    use std::vec;

    pub fn run() {
        // spawn()
        message_passing()
    }

    pub fn spawn() {
        // creating a new thread with `spawn`
        let spawned_thread = std::thread::spawn(|| {
            for i in 1..=10 {
                println!("hi number {i} from the spawned thread");
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        // wait for the spawned_thread to finish execution.
        // calling join blocks the thread currently running until the thread represented by the handle
        // terminates.
        spawned_thread.join().unwrap();
    }

    pub fn message_passing() {
        // Rust uses channels - A channel is a general programming concept by which data is sent
        // from one thread to another.
        // A channel has two halves: a transmitter and a receiver.
        // A channel is said to be closed if either the transmitter of receiver half is dropped.
        //

        // mpsc stands for multiple producer, single consumer. i.e a channel can have multiple
        // sending ends that produce values but only one receiving end that consumes those values.
        let (tx, rx) = std::sync::mpsc::channel();

        // `move` lets the thread take ownership of all memory used in the closure, that is owned by
        // the main thread.
        // Failure to move the data would lead to a compile-time error.
        std::thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        // The receiver has two useful methods: `recv` and `try_recv`.
        // `recv` will block the main thread's execution and wait until a value is sent down the
        // channel.
        // When the transmitter closes, recv will return an error to signal that no more values
        // will be coming.
        // `try_recv` method doesn't block, but will instead return a Result<T, E> immediately
        let received = rx.recv().unwrap();
        println!("Got: {}", received);

        // Sending multiple values
        {
            let (tx, rx) = std::sync::mpsc::channel();

            std::thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];
                for val in vals {
                    tx.send(val).unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            });

            for received in rx {
                println!("2 Got: {}", received);
            }
        }

        // Creating multiple producers by cloning the Transmitter
        {
            let (tx, rx) = std::sync::mpsc::channel();

            let tx1 = tx.clone();
            std::thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];

                for val in vals {
                    tx1.send(val).unwrap();
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            });

            //

            std::thread::spawn(move || {
                let vals = vec![
                    String::from("more"),
                    String::from("messanges"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            });

            //

            for received in rx {
                println!("3 Got: {}", received);
            }
        }
        //

        // Shared-state concurrency
        // This is where multiple threads access the same shared data.
        // Using Mutexes to Allow Access to Data from one thread at a time.
        // Mutex - abbreviation for Mutual Exclusion.
        // A mutex allows only one thread to access some data at any given time.
        // To access data in a mutex, a thread must first signal that it wans access by asking to
        // acquire the mutex's lock.
        // The lock is a data structure that is part of the mutex that keeps track of who currently
        // has exclusive access to the data.
        {
            let m = std::sync::Mutex::new(5);

            {
                // Mutex<T> is a smart pointer.
                // The call to lock() returns a smart pointer called MutexGuard wrapped in a
                // `LockResult` handled with unwrap in this case.
                // MutextGuard implements a `Deref` trait to point at our inner data.
                // The MutexGuard also implements a `Drop` trait which releases the lock
                // automatically when it goes out of scope.
                let mut num = m.lock().unwrap();
                *num = 6;
            }

            println!("4 m = {:?}", m);
        }

        //

        // Sharing a Mutex<T> between multiple threads
        //
        // Atomic Reference Counting with Arc<T>
        //
        // `Arc<T>` is type like Rc<T> that is safe to use in concurrent situations.
        // Atomics are an additional kind of concurrency primitive.
        // Arc<T> adds performance penalty inorder to make it thread safe.
        {
            let counter = std::sync::Arc::new(std::sync::Mutex::new(0));
            let mut handles = vec![];

            for _ in 0..10 {
                let counter = std::sync::Arc::clone(&counter);
                let handle = std::thread::spawn(move || {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            println!("5 Result: {}", *counter.lock().unwrap());

            // Note that if you are doing numerical operations, there are types simpler than
            // Mutex<T> types provided by the std::sync::atomic. These types provide safe
            // concurrent atomic access to primitive types.
        }

        // Extensible Concurrency with the Sync and Send Traits
        //
        // The `Send` marker trait indicates that ownership of values of the type implementing
        // Send can be transferred between threads. Almost all types in rust implement Send
        // ,except types such us Rc<T> since cloning it an transferring ownership would result
        // in multiple threads updating the reference count.

        // NOTE:
        // Implementing Send and Sync manually is sunsafe
    }
}
