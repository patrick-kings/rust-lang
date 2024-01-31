use std::{
    future::Future,
    pin::Pin,
    sync::{
        mpsc::{sync_channel, SyncSender},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
    thread,
};

use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
    FutureExt,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

// shared state between the futrue and the waiting thread
struct SharedState {
    // whether or not the sleep time has elapsed
    completed: bool,
    // The waker for the task that `TimerFuture` is running on.
    // The thread can use this after setting `complete=true` to tell `TimerFuture` task to wake up
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // check the shared state to see if the timer has already completed.
        let mut shared_state = self.shared_state.lock().unwrap();

        // if the thread has set `shared_state.completed = true`, we're done, otherwise, we clone
        // the `Waker` for the current task and pass it to `shared_state.waker` so that the thread
        // can wake the task back up.
        if shared_state.completed {
            return Poll::Ready(());
        } else {
            // Set waker so that the thread can wake up the current task when the timer has
            // completed, ensuring that the future is polled again and sees that completed=true.
            //
            shared_state.waker = Some(cx.waker().clone());
            return Poll::Pending;
        }
    }
}

// We have to update the waker every time the future is polled because the future may have moved to
// a different task with a different Waker. This will happen when futures are passed around between
// tasks after being polled.
impl TimerFuture {
    // create a new `TimerFuture` which will complete after the provided timeout.
    pub fn new(duration: std::time::Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // spawn the new thread
        let thread_shared_state = shared_state.clone();

        std::thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();

            // signal that the timer has completed and wake up the last task on which the future was
            // polled, if one exists.
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}

// Rust's Futures are lazy: they won't do anything unless actively driven to completion. One way to
// do that is to use .await inside an async function. The futures returned from the top level async
// function will need to be run by a `Future executor`.
//
// Future executors take a set of top-level Futures and run them to completion by calling poll
// whenever the Future can make progress.
// Typically, an executor will poll a future once to start off. When Futures indicate that they are
// ready to make progress by calling wake(), they are placed back onto a queue and poll is called
// again, repeating until the future has completed.
//
// The executor works by sending tasks to run over a channel. The executor will pull events off of
// the channle and run them. When a task is ready to do more work i.e it is awoken, it can schedule
// itself to be polled again by putting itself back onto the channel.
//
// The executor itself just needs the receiving end of the task channel. The user wil get a sending
// end so that they can spawn new futures. Tasks themselves are just fututes that can reschedule
// themselves, so we'll store them as a future paired with a sender that the task can use to
// requeue itself.

// Task executor that receives tasks off of a channel and runs them.
pub struct Executor {
    ready_queue: std::sync::mpsc::Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // Take the Future, and if it has not yet completed i.e is still Some, poll it in an
            // attempt to complete it.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // crate a localWaker from the task itself
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);

                // BoxFuture<T> is a type alias for Pin<Box<dyn Future<Output = T> + Send +
                // 'static>>.
                // We can get a Pin<&mut dyn Future + Send + 'static> from it by calling the
                // Pin::as_mut method.
                if future.as_mut().poll(context).is_pending() {
                    // We're not done processing the future, so put it back in its task to be run
                    // again in the future.
                    *future_slot = Some(future);
                }
            }
        }
    }
}

// `spawner` spawns new futures onto the task channel.
#[derive(Clone)]
pub struct Spawner {
    task_sender: std::sync::mpsc::SyncSender<Arc<Task>>,
}

/*
let's add a method to Spawner to make it easy to spawn new futures.
This method will take a future type, box it and create a new Arc<Task> with it inside which can be enqueued onto the
executor.
*/
impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

// a future that can reschedule itself to be polled by an executor.
struct Task {
    // In-progress future that should be pushed to completion.
    //
    // The `Mutex` is not necessary for correctness, since we only have one thread executing tasks
    // at once. However, Rust isn't smart enought to know that future is only mutated from one
    // thread, so we need to use the Mutex to prove thread-safety. A producton executor would not
    // need this, and could use `UnsafeCell` instea.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    // handle to place the task itself back onot the task queue.
    task_sender: SyncSender<Arc<Task>>,
}

// To poll futures, we'll need to create a Waker.
// When a Waker is created from Arc<Task>, calling wake() on it will cause a copy of the Arc to be
// sent onto the task channel. Our executor then needs to pick up the task and poll it.
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel so that it will be
        // polled again by the executor.
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    // Maximum number of tasks to allow queueing in the channel at once.
    // This is just to make `sync_channel` happy, and wouldn't be present in a real executor.
    const MAX_QUEUED_TASKS: usize = 10_000;

    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);

    (Executor { ready_queue }, Spawner { task_sender })
}
