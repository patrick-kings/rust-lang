// generics - abstract stand-ins for concrete types or other properties.
// we use generics to create definitions for items like function signatures or structs, which we
// can then use with many different concrete data types.

fn main() {
    println!("generic types");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    use_point();
}

//
pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// defining struct to use generic type parameters
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// we can specify constraints on generic types when defining methods on the type.
// We could for example, implement methods only on Point<f32> instances rather than on Point<T>
// instances with any generic type.
// We use the concrete type f32, meaning that we don't declare any types after impl
//This means that other instances of Point<T> where T is not of type f32 will not have this method
//  defined
//
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn use_point() {
    let _int = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 54.2 };
    let _int = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 54.2 };

    println!("p.x = {}", float.x());
}

// syntax for specifying generic type parameters, trait bounds and lifetimes all in one function

use std::fmt::Display;

pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
