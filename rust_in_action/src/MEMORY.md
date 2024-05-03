# Dynamic memory allocation

    At any given time, a running program has fixed number of bytes with which to get its work done. When the program would like more memory, it needs to ask for more from the OS. This is known as dynamic memory allocation

In unix the system allocator is called with `alloc()` while in windows, it is
`HeapAlloc()`. Freeing the memory in unix is `free()` while in windows it is
`HeapFree()`.

The allocation of memory on the heap is non-deterministic and hence a small
allocation could take as long as allocating a huge memory size or vice versa.

Some general strategies for minimizing heap allocations include:

    - Using arrays of uninitialized objects. Instead of creating objects from scratch as required, create a buld of those zeroed values. When the time to activate one of those objects comes, set its values to non-zero. This can be a very dangerous strategy because you're circumventing Rust's lifetime checks.

    - Using an alllocator that is tuned for your application's access memory profile. Memory allocators are often sensitive to the sizes where these perform best.

    - Investigat arena::Arena and arena::TypedArena. These allow objects to be created on the fly, but alloc() and free() are only called when the arena is created and destroyed.


# Virtual Memory

### Terminology

Page - A fixed-size block of words of real memory. Typically 4 kb in size for 64 bit operating systems.
Word - 
