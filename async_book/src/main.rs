mod timer;
use std::process::Output;

use futures::executor::block_on;
use futures::{future, select};

/*
async / .await is Rust's built-in tool for writing asynchronous functions that look like
synchronous code. async transforms a block of code into a state machine that implements a trait
called Future.
- Whereas calling a block function in a synchronous mehtod would block the whole thread, blocked
Futures will yield control of the thread, allowing other Futrues to run.

- To create an asynchronous function, you use the async fn syntax
- The value returned by async fn is a Future. For anything to happen, the Future needs to be run on an executor.



*/

fn main() {
    println!("async... .await");

    // block_on() blocks the current thread, and waits for the future to complete.
    block_on(async_main());

    let (executor, spawner) = timer::new_executor_and_spawner();

    spawner.spawn(async {
        println!("howdy");

        timer::TimerFuture::new(std::time::Duration::new(1, 0)).await;

        println!("done");
    });

    // Drop the spawner so that our executor knows it is finished and won't receive more incoming taks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy", pause and then print "done"
    executor.run();
}

async fn async_main() {
    let br = bar().await;

    println!("bar is  {}", br);

    // await
    let ls = learn_and_sing();
    let d = dance();

    /*
    - join! is like .wait but can wait for multiple futures concurrently.
    - If we are temporarily blocked in the learn_and_sing() future, the dance() future will take
    over the current thread. If dance() becomes blocked, learn_and_sing() can take back the
    thread. If bothe futures are blocked, then async_main() is blocked and will yield to the
    executor.

    NOTE: futures::join!() can only be used inside an async function.

    - join! is variadic, so any number of inputs can be sent.
    - The output is a tuple of the results from the async functions.
    */
    futures::join!(ls, d);

    // select!
    async {
        let mut a_fut = future::ready(4);
        let mut b_fut = future::ready(6);
        let mut total = 0;

        loop {
            select! {
                a = a_fut => total += a,
                b = b_fut => total += b,
                complete => break,
                default => unreachable!(), // never runs (futures are ready, then complete)
            };
        }
        assert_eq!(total, 10);
    }
    .await;
}
/*
mod tr_join {

    /*
    - For futres which return `Result`, consider using `try_join!` rather than `join!`. Since `join!` only completes once all subfutures have completed, It'll continue processing other futures even after on of its subfutures has returned an Err.

    - Unlike join!, try_join! will complete immediately if one of the subfutures returns an error.
     */

    use futures::try_join;

    async fn get_book() -> Result<Book, String> {
        /* ... */
        Ok(Book)
    }

    async fn get_music() -> Result<Music, String> {
        /* ... */
        Ok(Music)
    }

    async fn get_book_and_music() -> Result<(Book, Music), String> {
        let book_fut = get_book();
        let music_fut = get_music();
        try_join!(book_fut, music_fut)
    }
}
*/
mod slect {

    /*
    select!

    */

    use futures::{
        future::FutureExt, // for `.fuse()`

        pin_mut,
        select,
    };

    async fn task_one() {
        println!("task one")
    }
    async fn task_two() {}

    async fn race_tasks() {
        let t1 = task_one().fuse();
        let t2 = task_two().fuse();

        pin_mut!(t1, t2);

        // t1 and t2 wil be run concurrently. When either t1 or t2 finishes, the corresponding
        // handler will call println!, and the function will end without completing the remaining
        // task.
        //
        select! {
            ()= t1 => println!("task one completed first"),
            ()=t2 => println!("task two completed first"),
        }
    }
}

async fn foo() -> u8 {
    7
}
// you can also use async blocks which also return a value that implements the Future trait.
fn bar() -> impl futures::Future<Output = u8> {
    println!("async block");
    // This async block results in a type that implements `Future<Output = u8>`.
    async {
        let x: u8 = foo().await;
        x + 6
    }
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
