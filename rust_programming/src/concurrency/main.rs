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
fn spawn() {
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

    spawned_thread.join().unwrap();
}

fn main() {
    println!("fearless concurrency");

    spawn();

    println!("hello");
}
