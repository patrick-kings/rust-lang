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
