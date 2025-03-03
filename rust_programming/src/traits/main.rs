/*
* Traits define shared behavior.
* Types Implent traits using impl
* Default methods provied reusable logic
*
* Trait bounds restrict generic types to ensure they implement specific trais. This allows
* functions, structs and enums to work with only those stypes that have certain behaviors.
*/

trait Animal {
    fn name(&self) -> String;
}

// traits can extend other traits
trait Pet: Animal {
    fn is_domesticated(&self) -> bool;
    fn statement(&self);
}

trait Speak {
    fn speak(&self);
}

struct Dog;

impl Animal for Dog {
    fn name(&self) -> String {
        "Dog".to_string()
    }
}

impl Pet for Dog {
    fn is_domesticated(&self) -> bool {
        true
    }

    fn statement(&self) {
        println!("{} is domesticated {}", self.name(), self.is_domesticated())
    }
}

impl Speak for Dog {
    fn speak(&self) {
        println!("woof");
    }
}

// trait bounds
//  'T: Speak' means T must implement the Speak Trait.
fn make_speak<T: Speak>(animal: T) {
    animal.speak();
}

// for complex bounds, use a where clause instead of inline trait bounds.
// multiple traits can be used
fn describe<T, X>(item: T, animal: X)
where
    T: Speak + Pet,
    X: Animal,
{
    item.speak();
    println!("{}", animal.name())
}

// structs can use trait bounds to restrict the types of their generic fields.
struct Cat<T: Speak> {
    // Cat<T> can only be created with types that implement Speak, this ensures
    // that .speak() is always available
    pet: T,
}
impl<T: Speak> Cat<T> {
    fn make_noise(&self) {
        self.pet.speak()
    }
}

// using dynamic dispatch with dyn trait
// instead of static dispatch with generics, you can use trait objects (dyn trait)
// accepts a reference to any type implementing 'Speak'
fn make_speech(animal: &dyn Speak) {
    animal.speak()
}

// Implementing traits for existing types
trait Double {
    //
    // associated types, or type alias, they make generics easier
    type Item;

    fn double(&self) -> Self;

    // default method, which prints the initial value
    fn print_value(&self) {
        println!("{}", "default method")
    }
}

impl Double for i32 {
    type Item = i32;

    fn double(&self) -> Self {
        self * 2
    }

    fn print_value(&self) {
        println!("initial value is: {}", self)
    }
}

fn main() {
    5.print_value(); // the default method in the trait, prints 5

    println!("{}", 5.double()); // prints 10

    let dog = Dog;
    dog.statement();

    make_speech(&dog)
}
