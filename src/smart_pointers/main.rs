//
// The Reference counting smart pointer type -> This pointer enables you to allow data to have
// multiple owners by keeping track of the number of owners, and when no owners remain, the data is
// cleaned.

// Rust with its concept of ownership and borrowing has an additional difference between references
// and smart pointers -: while references only borrow data, in many cases, smart pointers own the
// data they point to.

// smart pointers include but not limited -: `String` `Vec<T>`. These types count as smart
// pointers because they own some memory and allow you to manipulate it. The also have metadata and
// extra capabilities or guarantees i.e `String` stores its capacity as metadata and has the extra
// ability to ensure its data will always be valid UTF-8.

// Smart pointers are usually implemented using structs. unlike an ordinary struct, smart pointers
// implement the `Deref` and `Drop` traits.
// The `Deref` trait allows an instance of the smart pointer struct to behave like a reference so
// you can write your code to work with either references or smart pointers.
// The `Drop` trait allows you to customize the code that's tun when an instance of the smart
// pointer goes out of scope.

// Common Smart pointers are -:
//      `Box<T>` for allocating values on the heap
//      `Rc<T>` a reference counting type that enables multiple ownership
//      `Ref<T>` and `RefMut<T>` accessed through `RefCell<T>` a type that enables the borrowing
//      rules at runtime instead of compile time.

//          Box<T>
// Boxes allow you store data on the head rather than the stack. What remains ont he stack is the
// pointer to the heap data.
// Boxes don't have performance overhead. other than storing their data on the heap instead of on
// the stack but they don't have many capabilities either.
// The are useful in these situations -:
//      When you have a type whose size can't be known at compile time and you want to use a value
//      of that type in a context that requires an exact size.
//      When you have a large amount of data and you want to transfer ownership but unsure the data
//      won't be copied when you do so.
//      When you want to own a value and you care only that it's a type that implements a
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
// The new function mimics the new function in the std::Box type
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// implement the `Deref` trait for `MyBox` type
// The type `Target = T` defines an associated type for the `Deref` trait to use.
// The body is filled with the `deref` method with &self.o so deref return a reference to the value
// wi want to access with * operator. `0` access the first value in a tuple struct (MyBox)
impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn use_my_box() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);
}

fn main() {
    println!("smart pointers");

    smart_pointer_as_regular_references();

    use_my_box();
}
