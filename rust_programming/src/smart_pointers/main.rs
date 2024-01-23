//
// The Reference counting smart pointer type -> This pointer enables you to allow data to have
// multiple owners by keeping track of the number of owners, and when no owners remain, the data is
// cleaned.

// Rust with its concept of ownership and borrowing has an additional difference between references
// and smart pointers -: while references only borrow data, in many cases, smart pointers own the
// data they point to.

// smart pointers include but not limited -: `String` `Vec<T>`. These types count as smart
// pointers because they own some memory and allow you to manipulate it. They also have metadata and
// extra capabilities or guarantees i.e `String` stores its capacity as metadata and has the extra
// ability to ensure its data will always be valid UTF-8.

// Smart pointers are usually implemented using structs. unlike an ordinary struct, smart pointers
// implement the `Deref` and `Drop` traits.
// The `Deref` trait allows an instance of the smart pointer struct to behave like a reference so
// you can write your code to work with either references or smart pointers.
// The `Drop` trait allows you to customize the code that's ran when an instance of the smart
// pointer goes out of scope.

// Common Smart pointers are -:
//      `Box<T>` for allocating values on the heap
//      `Rc<T>` a reference counting type that enables multiple ownership
//      `Ref<T>` and `RefMut<T>` accessed through `RefCell<T>` a type that enables the borrowing
//      rules at runtime instead of compile time.

//          Box<T>
// Boxes allow you store data on the heap rather than the stack. What remains on he stack is the
// pointer to the heap data.
// Boxes don't have performance overhead. other than storing their data on the heap instead of on
// the stack, but they don't have many capabilities either.
// The are useful in these situations -:
//  - When you have a type whose size can't be known at compile time and you want to use a value
//      of that type in a context that requires an exact size.
//  - When you have a large amount of data and you want to transfer ownership but unsure the data
//      won't be copied when you do so.
//  - When you want to own a value and you care only that it's a type that implements a
//      particular trait rather than being of a specific type.

fn _boxes() {
    // we define a variable b to have a value of `Box` that points to the value 5 which is
    // allocated on the heap.
    // When a `Box` goes out of scope, it will be deallocated, The deallocation happens both for
    // the `Box` stored on the stack and the data it points which is stored on the heap.
    let b = Box::new(5);
    println!("b = {b}");
}

// Recursive types with Boxes
// A value of recursive type can have another value of the same type as part of itself.
// Recursive types pose an issue because at compile time, Rust needs to know how much space a type
// takes up, however, the nesting of values of recursive types could theoretically continue
// infinitely, so Rust can't know how much space the value needs.
// Because boxes have a known size, we can enable recursive types by inserting a box in the
// recursive type definition. i.e `cons list`
// A cons list is a data structure that comes from the lisp programming language and its dialects
// and is made up of nested pairs, it's similar to a linked list.
fn _cons_list() {
    // Because `Box<T>` is a pointer, Rust always knows how much space a `Box<T>` needs: (a
    // pointer's size doesn't change based on the amount of data it's pointing to.) This means that
    // we can put a Box<T> inside the Cons variant instead of another `List` value directly.

    // The Box<T> will point to the next `List` value that will be on the heap rather than inside
    // the `Cons` variant.
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    // This `Cons` variant needs the size of an i32 plus the space to store the box's pointer data.
    // The Nil variant stores no values, so it needs less space than the `Cons` variant.
    // By using a box we have broken the infinite, recursive chain, so the compiler can figure out
    // the size it needs to store a List.
    use List::{Cons, Nil};
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// The `Deref` trait allows you to customize the behaviour of the dereference operator `*`.
fn smart_pointer_as_regular_references() {
    let x = 10;
    let x1 = &x;
    assert_eq!(10, *x1, "testing if x is equal to {x}");

    let y = Box::new(5);
    assert_eq!(5, *y, "testing if y is equal to {y}"); // using the dereference operator `*`
}

// defining our own smart pointer
// The Box<T> type is defined as a tuple struct with one element
// The new function mimics the std::Box type
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// implement the `Deref` trait for `MyBox` type
// The type `Target = T` defines an associated type for the `Deref` trait to use.
// The body is filled with the `deref` method with &self.o so deref return a reference to the value
// we want to access with * operator. `0` access the first value in a tuple struct (MyBox)
impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// The `deref` method gives the compiler the ability to take a value of any type that implements
// `Deref` and call the `deref` method to get a `&` reference that it knows how to dereference.
//  `*y` translates to `*(y.deref())` behind the scenes.
fn use_my_box() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);
}

// Implicit Deref Coercions with functions and methods.
//
// Deref coercion converts a reference to a type that implements the `Deref` trait into a reference
// to another type. e.g deref coercion can convert `&String` to `&str` because `String` implements
// the Deref trait such that it return `&str`.
// Deref coercion happens automatically when we pass a reference to a particular type's value as an
// argument to a function or method that doesn't match the parameter type in the function or
// method definition. A sequence of calls to the deref method converts the type we provided into
// the type the parameter needs.
fn deref_coercion() {
    println!("deref coercion");

    let hello = |name: &str| {
        println!("hello, {name}");
    };

    let m = MyBox::new(String::from("Rust"));

    hello(&m);
}

// `Drop` Trait
// The `Drop` trait lets us customize what happens when a value is about to go out of scope.
// The Drop trait can be implemented on any type which can be used to release resources like files
// or network connections.
struct CustomSmartPointer {
    data: String,
}

impl std::ops::Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

fn use_drop_trait() {
    // Rust calls drop after this (`use_drop_trait`) function exits.
    // calling drop manually produces a compile time error, this is because Rust will still call it
    // when the value leaves the scope.
    println!("The Drop Trait");

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    // if we want to drop a value early, we have to use the `std::mem::drop` function.
    std::mem::drop(c);

    println!("CustomSmartPointers droped");
}

// Rc<T> - The Reference Counted smart Pointer
//
// In the majority of cases, ownership is clear: you know exactly which variable owns a given value.
// However, there are cases when a single value might have multiple owners.
// For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it.
// A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it and so has no owners.
// In such a case, you have to enable multiple ownership explicitly by using `Rc<T>` type
// (Reference counting).
// `Rc<T>` keeps track of the number of references to a value to determine whether or not the valeu
// is still in use. If there are no zero references to a value, the value can be cleaned up without
// any refeences becoming invalid.
//
// We use the Rc<T> type when we want to allocate some data on the head for multiple parts of our
// program to read and we can't determine at compile time which part will finish using the data
//
// Rc<T> is only for use in single-threaded scenarios.
use std::rc::Rc;
enum List2 {
    Cons(i32, std::rc::Rc<List2>),
    Nil,
}

pub fn use_rc() {
    let a = Rc::new(List2::Cons(
        5,
        Rc::new(List2::Cons(10, Rc::new(List2::Nil))),
    ));

    println!("count after creating a = {}", Rc::strong_count(&a));
    // by cloning the Rc that is `a` is holding, we increase the number of references from one to
    // two and letting `a` and `b` share ownership of the data in that Rc<List2>
    // Everytime we call `Rc::clone()`, the refernce count to the data within the `Rc<List2>` will
    // increase an the data won't be cleaned up unless there are zero refernces to it.
    // The call to `Rc::clone()` doesn't perfom a deep copy like `a.clone()` does instead it only
    // increments the reference count which takes less time.
    let _b = List2::Cons(3, Rc::clone(&a));
    // print the refernce count each time we call `clone`
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = List2::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

pub fn _use_refcell() {
    // RefCell<T> and the Interior Mutability Pattern
    //
    // Interior Mutability is a design pattern in Rust that allows you to mutate data even when there
    // are immutable references to that data, normally, this action is disallowed by the borrowing
    // rules.
    // To mutate data, the pattern uses `unsafe` code inside a data structure to bend Rust's usual
    // rules that govern mutation and borrowing.
    // Unsafe code indicates to the compiler that we are checking the rules manually instead of relying
    // on the compiler to check them for us.
    //
    // We can use types that use the interior mutability pattern only when we can ensure that the
    // borrowing rules will be followed at runtime, even though the compiler can't guarantee that.

    // Enforcing Borrowing Rules at Runtime with `RefCell<T>`
    //
    // Unlike `Rc<T>`, the `<RefCell<T>` type represents single ownership over the data it holds.
    // The `RefCell<T>` varies from other smart pointers by having different rules, namely :
    //  - At any given time, you can have either(but not both), one mutable refernce or any number of
    //      immutable refernces.
    //  - References must always be valid.
    //
    //Unlike `Box<T>` where borrowing rules' invariants are checked at runtime, with `RefCell<T>`, these invariants
    //are checked at runtime. Therefore breaking these rules means that you program will panic and exit
    //during runtime.
    //
    // `RefCell<T>` is only for use in single-threaded scenarios.
}

fn main() {
    println!("smart pointers");

    smart_pointer_as_regular_references();

    use_my_box();

    deref_coercion();

    use_drop_trait();

    use_rc();
}
