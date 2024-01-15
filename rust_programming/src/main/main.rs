fn main() {
    // guessing_game();
    // concepts();
    // let (s1, int1) = returning_value();
    // traits();
    // optiions();
    // let value: u8 = value_in_cents(Coin::Quarter(UsState::Alaska));
    // println!("{}", value);

    // call a lib
    my_lib::hello_lib();

    catch_all_pattern();
}

fn guessing_game() {
    println!("Guess the number");

    use rand::Rng;
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess: String = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // convert guess to unsigned int32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn concepts() {
    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // scalar types in rust
    // integers, floating-point numbers, booleans and characters

    // integers
    // signed integers start with i
    // i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize

    // hexadecimal 0xff
    // Decimal 98_222
    // octal 0o77
    // binary 0b1111_0000
    // byte (u8 only) b'A'

    // floating point Types
    let _float: f32 = 2.0;
    let _float: f64 = 3.0;

    // booleans
    let _t: bool = true;
    let _f: bool = false;

    // character type
    // use single quotes for characters
    let _c: char = 'z';
    let _heart_eyed_cat: char = '😻';

    // compound types
    // tuple type
    let tup: (i32, f64, u8, char, bool) = (500, 2.6, 2, 'c', true);
    let (_x, y, _z, _c, _b) = tup;
    println!("y is : {}", y);

    // arrays
    let arr = [1, 2, 3, 4, 5];
    let arr2: [u32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr2[0]);

    // if expression
    if arr[0] > 0 {
        println!("true");
    } else {
        println!("false");
    }

    // loop over expression
    // loop{}

    // returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);

    // loop labels
    // if you have loops within loops, break and continue apply to the innermost loop at that point.
    // you can optionally specify a loop label on a loop than can be used with a break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop.
    let mut count = 0;

    // counting_up label. labels start with a single quote
    'counting_up: loop {
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // break out of the outer 'counting_up loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // conditional loops with while
    let mut num = 3;

    while num != 0 {
        num -= 1;
        println!("{}", num);
    }

    // loops with for
    let arr1 = [10, 20, 5, 6, 8];

    for element in arr1 {
        println!("{}", element);
    }

    // using rev to reverse order
    for element in (1..5).rev() {
        println!("{}", element);
    }

    // ownership
    // because strings are compund data types, they are stored in the heap both s1 and s2 will point to the same location in memory.
    // s1 is moved to s2 and s1 is discarded. This prevents double free error causes when s1 and s2 goes out of scope and there is an attemp to free their memory.
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    // cloning data
    let s3: String = s2.clone();
    println!("{}", s3);

    // fundamental types support copying since they are stored in the stack i.e integers, booleans, floats, characters, tuples (only if they contain types that implement the fundamental types)
    let x = 4;
    let y = x;
    println!("{} {}", y, x);
}

fn returning_value() -> (String, usize) {
    let str: String = String::from("hello");
    return (str, 8);
}

// you must declare mut in function parameters in order to modify the arguments
fn returning_value2(str: &mut String) {
    str.push_str("world");
}

// slice type
fn slice(str: &String) -> usize {
    let bytes = str.as_bytes();

    // return the index at the end of the bytes, marked by ''
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    // these are equal to str[..]
    if str[0..] == str[0..str.len()] {
        return 1;
    }

    let x = [3, 4];
    assert_eq!(x, [3, 4]);

    return str.len();
}

// structs
fn structs() {
    struct User {
        active: bool,
        username: String,
        sign_in_count: u32,
    }

    let _user = User {
        active: true,
        username: String::from("Doe"),
        sign_in_count: 0,
    };

    // using tuple structs without named fields to create different types
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

// Traits
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implement a trait for Rectangle struct
// associated functions. all functions defined within an impl block are called associated functions because they are associated with the type named after the impl.
// we can define associated functions that don't have self as the first parameter and thus are not methods because the don't need an instance of the type to work with. e.g String::from function.
// associated functions that aren't methods are often used for constructors that will return a new instance of the struct.
// each struct can have multiple impl blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // an associated function which is not a method.
    // to call this function we use ::. i.e Rectangle::square(4);
    fn square(size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        };
    }
}

fn traits() {
    let rect: Rectangle = Rectangle {
        width: 3,
        height: 2,
    };

    // putting the specifier :? inside curly brackets tels println that we want to us an output formate called Debug. The debug trait  enables us to print our struct in a way that can help with debugging. for this trait to be implemented we add an outer attribute to the struct `#[derive(Debug)]`
    println!("{:?}", rect);

    dbg!(&rect);

    println!("area is {}", rect.area());

    //
    let _sq = Rectangle::square(5);
}

// enums
enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// methods can also be implemented for enums
impl IpAddr {
    fn get_ip(&self) {
        // method body
    }
}

fn enums() {
    let _four: IpAddr = IpAddr::V4(String::from("127.0.0.1"));
    let _six: IpAddr = IpAddr::V6(String::from("::1"));

    //
    let _home = IpAddr2::V4(127, 0, 0, 1);
    let _loopback = IpAddr2::V6(String::from("::1"));
}

// option enum
// The option type encodes the common scenario in which a value could be something or nothing.
// rust does not have nulls.

fn optiions() {
    let num = Some(5);
    let _char = Some('c');

    let absent_num: Option<i32> = None;

    if absent_num.is_none() {
        println!("absent_num is none");
    }

    match num {
        Some(num) => println!("num is {}", num),
        None => println!("num is None"),
    }
}

// match control flow construct

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    California,
}
// returns the value of the coin in u8 depending on which coin is in the argument
fn value_in_cents(coin: Coin) -> u8 {
    return match coin {
        Coin::Penny => {
            println!("penny");
            1
        }
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        }
    };
}

// catch-all
// by adding other, all match cases are covered, alternatively we can use an _ to signify that we would like to discard the other matches. e.g `_ =>();`
fn catch_all_pattern() {
    let dice_roll = 9;

    match dice_roll {
        3 => println!("{}", 3),
        4 => println!("{}", 4),
        5 => println!("{}", 5),
        other => println!("{}", other),
    }
}

// concise control flow with if let
// this avoids situations where you need to exhaust a match case.
fn if_let() {
    let config_max: Option<u8> = Some(3u8);

    if let Some(max) = config_max {
        println!("max is {}", max);
    }
}

fn error_handling() {
    let greeting_file_result = std::fs::File::open("hello.txt");

    let _greeting_file: std::fs::File = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match std::fs::File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problecm createing the file: {:?}", e),
            },
            other => panic!("problem opening the file: {:?}", other),
        },
    };
}

// the ? placed after a result value is defined to work in almost the same way as a match
fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut username_file = std::fs::File::open("hello.txt")?;

    let mut username = String::new();

    use std::io::Read;
    username_file.read_to_string(&mut username)?;
    // or
    // std::fs::File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
