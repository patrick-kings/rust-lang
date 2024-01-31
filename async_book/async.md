# Async

Asynchronous programming, or async is a concurrent programming model that let
you run a number of concurrent tasks on a small number of os threads.

### Async and other concurrency models

Types of concurrency models :-

- OS threads - They don't require any changes to the programming model, which
  makes it very easy to express concurrency. However, synchronizing between
  threads can be difficult and the performance overhead is large. Thread pools
  can mitigate some of these costs, but not enough to support massive IO-bound
  workloads.
- Event-driven Programming - It uses callbacks. It can be very performant, but
  tends to result in a verbose, "non-linear" control flow. Data flow and error
  propagation is often hard to follow.
- Coroutines - like threads, don't require changes to the programming model,
  which makes them easy to use. Like async, they can also support a large number
  of tasks. However, they abstract away low-level details that are important for
  systems programming and custom runtime implementors.
- The actor model - It divides all concurrent computation into units called
  actors which communicate through fallible message passing, much like in
  distributed systems. The actor model can be efficiently implemented, but it
  leaves many practical issues unanswered, such as flow control and retry logic.

### Async in Rust vs other languages.

- <b>Futures are inert</b> in Rust and make progress only when polled. Dropping
  a future stops it from making further progress.
- <b>Async is zero-cost</b> in Rust, which means that you only pay for what you
  use. Specifically, you can use async without heap allocations and dynamic
  dispatch, which is great for performance! This also lets you use async in
  constrained environments, such as embedded systems.
- <b>No built-in runtime</b> is provided by Rust, runtimes are provided by
  community maintained crates.
- <b>Both single and multithreaded</b> runtimes are available in Rust, which
  have different strengths and weaknesses.

### Async vs threads in Rust

The primary alternative to async in Rust is using OS threads, either directly
through `std::thread` or indirectly through a thread pool. Migrating from
threads to async or vice versa typically requires major refactoring work, both
in terms of implementation and (if you are building a library) any exposed
public interfaces. As such, picking the model that suits your needs early can
save a lot of development time.

<b>OS threads </b> are suitable for a small number of tasks, since threads come
with CPU and memory overhead. Spawning and switching between threads is quite
expensive as even idle threads consume system resources. A thread pool library
can help mitigate some of those costs, but not all. However, threads let you
reuse existing synchronous code without significant code changes - no particular
programming model is required. In some operating systems, you can change the
priority of a thread, which is useful for drivers and other latency sensitive
applications.

**Async** provides significantly reduced CPU and memory overhead, especially of
workloads with a large amount of IO-bound tasks, such as servers and databases.
All else equal , you can have orders of magnitude more tasks than OS threads,
because an async runtime uses a small amount of (expensive) threads to handle a
large amount of (cheap) tasks. However, async Rust results in larger binary
blobs due to the state machines generated from async functions and since each
executable bundles an async runtime.

**NOTE :** asynchronous programming is not better that os threads, but
different. If you don't need async for performance reasons, threads can often be
the simpler alternative.

# The Future trait

A future is an asynchronous computation that can produce a value (although the
value may be empty, e.g. ()).

```rust
// a simplified version of a future trait.
trait SimpleFuture{
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<self::Output>;
}

enum Poll<T>{
    Ready(T),
    Pending,
}
```

Futures can be advanced by calling the poll function, which will drive the
future as far towards completion as possible. If the future completes, it
returns `Poll::Ready(resut)`. If the future is not able to complete yet, it
returns `Poll::Pending` and arranges for the `wake()` function to be called when
the `Future` is ready to make more progress. When `wake()` is called, the
executor driving the `Future` will call poll again so that the Future can make
more progress. Without `wake()`, the executor would have no way of knowing when
a particular future could make progress, and would have to be constantly polling
every future. With `wake()`, the executor knows exactly which futes are ready to
be polled. This model of `Futures` allows for composing together multiple
asynchronous operations without needing intermediate allocations. Running
multiple futures at once or chaining futures together can be implemented via
allocation-free machines e.g

```rust
/// A SimpleFuture that runs two other futures to completion concurrently.
///
/// Concurrency is achieved via the fact that calls to `poll` each future
/// may be interleaved, allowing each future to advance itself at its own pace.
pub struct Join<FutureA, FutureB> {
    // Each field may contain a future that should be run to completion.
    // If the future has already completed, the field is set to `None`.
    // This prevents us from polling a future after it has completed, which
    // would violate the contract of the `Future` trait.
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        // Attempt to complete future `a`.
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        // Attempt to complete future `b`.
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            // Both futures have completed -- we can return successfully
            Poll::Ready(())
        } else {
            // One or both futures returned `Poll::Pending` and still have
            // work to do. They will call `wake()` when progress can be made.
            Poll::Pending
        }
    }
}
```

<i><b> These examples show how the future trait can be used to express
asynchronous control flow without requiring multiple allocated objects and
deeply nested callbacks.
</b></i>

### The real `Future` trait

```rust
trait Future{
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)-> Poll<Self::Output;
}
```

- The first change is that our self type is no longer &mut self, but has changed
  to Pin<&mut Self>. Pin allows us to create futures that are immovable.
  Immovable objects can store pointers between their fields,
  e.g.`struct MyFut { a: i32, prt_to_a: *const i32 }` Pinning is necessary to
  enable async/await.
- Secondly, `wake: fn()` has changed to `&mut Context<'_>`. The Context type
  provides access to a value of type `Waker` which can be used to wake up a
  specific task.

## Task Wakeups with Waker

`Waker` provides a `wake()` method that can be used to tell the executor that
the associated task should be awoken. When `wake()` is called, the executor
knows that the task associated with the `Waker` is ready to make progress, and
its future should be polled again. `Waker` also implements `clone()` so that it
can be copied around and stored.

## Executors and System IO

In a situation where a future is reading available data on a socket, the future
will read until there is no more data , then it will yield to the executor
requesting that its task be awoken when the socket becomes readable again. There
are two options to do this :-

- option one is to have a thread that continually checks whether socket is
  readable, calling wake() when appropriate. However, this approach is
  inefficient, requiring a separate thread for each blocked IO future would
  greatly reduce the efficiency of async code.
- Option two which is the solution is to integrate with an IO-aware system
  blocking primitive, such as `epoll` on Linux, `kqueue` on FreeBSD and Mac OS,
  `IOCP` on Windows and `port` on Fuchsia using the crate
  [mio](https://github.com/tokio-rs/mio). These primitives allow a thread to
  block on multiple asynchronous IO events, returning once one of the events
  completes.

<i> An example of IO-aware system blocking primitive API </i>

```rust
struct IoBlocker {
    /* ... */
}

struct Event {
    // An ID uniquely identifying the event that occurred and was listened for.
    id: usize,

    // A set of signals to wait for, or which occurred.
    signals: Signals,
}

impl IoBlocker {
    /// Create a new collection of asynchronous IO events to block on.
    fn new() -> Self { /* ... */ }

    /// Express an interest in a particular IO event.
    fn add_io_event_interest(
        &self,

        /// The object on which the event will occur
        io_object: &IoObject,

        /// A set of signals that may appear on the `io_object` for
        /// which an event should be triggered, paired with
        /// an ID to give to events that result from this interest.
        event: Event,
    ) { /* ... */ }

    /// Block until one of the events occurs.
    fn block(&self) -> Event { /* ... */ }
}

let mut io_blocker = IoBlocker::new();
io_blocker.add_io_event_interest(
    &socket_1,
    Event { id: 1, signals: READABLE },
);
io_blocker.add_io_event_interest(
    &socket_2,
    Event { id: 2, signals: READABLE | WRITABLE },
);
let event = io_blocker.block();

// prints e.g. "Socket 1 is now READABLE" if socket one became readable.
println!("Socket {:?} is now {:?}", event.id, event.signals);
```

## async / .await

There are two main ways to use async: - async fn - async blocks Each returns a
value that implements the Future trait:

```rust
// `foo()` returns a type that implements `Future<Output = u8>`.
// `foo().await` will result in a value of type `u8`.
async fn foo() -> u8 { 5 }

fn bar() -> impl Future<Output = u8> {
    // This `async` block results in a type that implements
    // `Future<Output = u8>`.
    async {
        let x: u8 = foo().await;
        x + 5
    }
}
```

### async liftimes

async functions which take references or other non-'static arguments return a
Future which is bounded by the lifetiem of the arguments:

```rust
// This function:
async fn foo(x: &u8) -> u8 { *x }

// Is equivalent to this function:
fn foo_expanded<'a>(x: &'a u8) -> impl futures::Future<Output = u8> + 'a {

}
```

This means that the future returned from an async fn must be .await ed while its
non-'static arguments are still valid. In the commin case of .wait ing the
future immediately after calling the function (as in foo(&x).await), this is not
an issue, However, if storing the future or sending it over to another task or
thread, this may be an issue.

One common workaround for turning an async fn with references-as-arguments into
a 'static future is to bundle the arguments with the call to async fn inside an
async block:

```rust
fn good()-> imple Future<Output =u8> {
    async {
        let x = 7;
        borrow_x(&x).await
    }
}
```

By moving the argument into the async block, we extend its lifetime to match
that of the Future returned from the call to good.

### async move

async blocks and closures allow the move keyword, much like normal closures. An
async move block will take ownership of the variables it references, allowing it
to outlive the current scope, but giving up the ability to share those variables
with other code:

```rust
// multiple different async blocks can access the same local variable so long as they're executed within the variable's scope
async fn blocks(){
    let my_string = "foo".to_string();

    let future_one = async {
        println!("{my_string}");
    };

    let future_two = async {
        println!("{my_string}");
    };

    // Run both futures to completion, printing "foo" twice:
    let ((), ()) = futures::Join!(future_one, future_two);
}

// async move block:
//
// only one async move block can access the same captured variable, since captures are moved into the Future generated by the async move block.
// However, this allows the Future to outlive the original scope of the varibale:
fn move_blocK() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();

    async move {
        println!("{my_string}");
    }
}
```

### .awaiting on a multithreaded Executor

<b><i> NOTE that when using a multithreaded Future Executor, a future may move
between threads, so any variables used in async bodies must be able to travel
between threads, as any `.await` can potentially result in a switch to a new
thread. This means that it is not safe to use `Rc`, `&RefCell` or any other
types that don't implement the `Send` trait, including referencesto types that
don't implement the `Sync` trait.</i></b>

<b><i> Caveat: It is possible to use these types as long as they aren't in scope
during a call to `.await`.</i></b>

<b><i> Similarly, it isn't a good idea to hold a traditional non-futures-aware
lock acress an .await, as it can couse the threadpool to lock up: one task could
take out a lock, .await and yield to the executor, allowing another task to
attempt to take the lock and cause a deadlock. To avoid this, use the `Mutex` in
`futures::lock` rather than the one from `std::sync`.</i> </b>

## Pinning

To poll futures, they must be pinned using a special type called Pin<T>. Pinning
makes it possible to guarantee that an object implementing !Unpin won't ever be
moved.

TODO:

## Executing multiple Futures at a time.

