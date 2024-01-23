// In this chapter:
// closures
// iterators
//

// Rust closures are anonymous function you can save in a variable or pass as arguments to other
// functions. You can create the closure in one place and then call the closre elsewhere to
// evaluate it in a different context. Unlike functions, closures can capture values from the scope
// in whitch they're defined.
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    Shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // closure
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.Shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        Shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let _opt: Option<i32> = None;
    let _opt2: Option<std::string::String> = Some("a string".to_string());

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    //
    let _expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        return num;
    };

    // a closure with no inputs
    let _ = || println!("a closure");
    borrow_mutability()
}

fn borrow_mutability() {
    let mut list = vec![1, 2, 3, 4];
    println!("before defining the closure {:?}", list);

    // The variable `borrows_mutability` binds to the closure
    // the closure borrows the list variable the returns it after the call.
    let mut borrows_mutability = || list.push(7);

    borrows_mutability();
    println!("after calling the closure {:?}", list);

    // if you want to force the closure to take ownership of the values it uses, you can use the
    // `move` keyword before the parameter list.
    // this technique is mostly useful when passing a closure to a new thread to move the data so
    // that it's owned by the new thread.
    std::thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // The way a closure captures and handles values from the environment affects which traits the
    // closure implements, and traits are how functions and structs can specify what kinds of
    // closures they can use.
    // Closures will automatically implement any or all of these traits, depending on how the
    // closure's body handles the values.
    //  Trait fnOnce -> applies to closures that can be called once. All closures implement this
    //      trait because all closures can be called.
    //  Trait FnMut -> applies to closures that don't move captured values out of their body, but
    //      that might mutate the captured values. The closures can be called more than once.
    //  Trait fn -> applies to closures that don't move captures values out of their body and don't
    //      mutate captured values, as well as closures that capture nothing from their
    //      environment. These closures can be called more than once without mutating their
    //      environment which is important in cases such as calling a closure multiple times
    //      concurrently.
}

// sample of Option type from the `std` library
// T is the generic type representing the type of the value in the `Some` variant of an `Option`.
// That type T is also the return type of the `unwrap_or_else` function.
// e.g Option<String> will get a String.
// we specify a trait bound for the generic type `F` of `FnOnce()` which means that `F` must be
// able to be called once, take no arguments and return a `T`.
/*impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
*/

// ITERATORS
// All iterators implement a trait named `Iterator` that is defined in the standard library.
// `type Item` and `self::Item` are associated types which
// pub trait Iterator {
//     type Item;
//      fn next (&mut self) -> Option<self::Item>;
// }
