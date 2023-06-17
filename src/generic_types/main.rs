// generics - abstract stand-ins for concrete types or other properties.

fn main() {
    println!("generic types");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

//
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
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

fn use_point() {
    let int = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 54.2 };
}
