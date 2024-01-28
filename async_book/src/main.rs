use futures::executor::block_on;
// async / .await is Rust's built-in tool for writing asynchronous functions that look like
// synchronous code. async transforms a block of code into a state machine that implements a trait
// called Future.
// - Whereas calling a block function in a synchronous mehtod would block the whole thread, blocked
// Futures will yield control of the thread, allowing other Futrues to run.
//
// - To create an asynchronous function, you use the async fn syntax
// - The value returned by async fn is a Future. For anything to happen, the Future needs to be
// run on an executor.
fn main() {
    println!("async... .await");

    // block_on() blocks the current thread, and waits for the future to complete.
    block_on(async_main());
}

async fn async_main() {
    // await
    let ls = learn_and_sing();
    let d = dance();

    // - join! is like .wait but can wait for multiple futures concurrently.
    // - If we are temporarily blocked in the learn_and_sing() future, the dance() future will take
    // over the current thread. If dance() becomes blocked, learn_and_sing() can take back the
    // thread. If bothe futures are blocked, then async_main() is blocked and will yield to the
    // executor.
    // NOTE: futures::join!() can only be used inside an async function.
    futures::join!(ls, d);
}

// .await
async fn learn() -> String {
    println!("learn");

    std::thread::sleep(std::time::Duration::from_secs(1));
    return "song".to_string();
}

async fn sing(song: String) {
    println!("sing {song}");
    std::thread::sleep(std::time::Duration::from_secs(1));
}

async fn learn_and_sing() {
    // wait until the song has been learned before singing it.
    // We use .await rather than block_on to prevent blocking the thread, which makes it possible
    // to dance at the same time.
    let song = learn().await;

    println!("learn and sing");

    sing(song).await;
    std::thread::sleep(std::time::Duration::from_secs(1));
}

async fn dance() {
    println!("dance");
    std::thread::sleep(std::time::Duration::from_secs(1));
}
