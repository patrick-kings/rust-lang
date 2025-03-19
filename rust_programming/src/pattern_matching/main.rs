fn main() {
    println!("patterns and matching");

    patterns_matching::run();
}

mod patterns_matching {
    pub fn run() {
        // fn_parameters();
        patterns();
    }

    pub fn if_let() {
        let favaourite_color: Option<&str> = None;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favaourite_color {
            println!("Using your favourite color: {}", color);
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        }
    }

    pub fn while_let() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    pub fn for_loops() {
        let v = vec!["a", "b", "c"];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    pub fn fn_parameters() {
        // Destructure a tuple in function arguments
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({} {})", x, y);
        }

        let point = (3, 5);
        print_coordinates(&point);
    }

    // Refutability: Whether a pattern might fail to match
    //
    // Patterns come in two forms: refutable and irrefutable.
    // Patterns that will match for any possible value passed are irrefutable. e.g let x = 5;
    // since x matches anything and therefore cannot fail to match
    // Patterns that can fail to match for some possible value are refutable. e.g Some(x), if the
    // value of x is None rather than Some, the Some(x) pattern will not match.
    //

    pub fn patterns() {
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            4..=6 => println!("4 through 6"),
            _ => println!("anything"),
        }

        let y = 'c';

        match y {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }

        {
            // Destructuring structs
            //
            // breaking apart values with let
            struct Point {
                x: i32,
                y: i32,
            }

            let p = Point { x: 0, y: 7 };

            let Point { x: a, y: b } = p;
            assert_eq!(0, a);
            assert_eq!(7, b);

            // or with the shorthand
            let Point { x, y } = p;
            assert_eq!(0, x);
            assert_eq!(7, y);

            // or using match
            match p {
                Point { x, y: 0 } => println!("on the x axis at {x}"),
                Point { x: 0, y } => println!("On the y axis at {y}"),
                Point { x, y } => {
                    println!("On neither axis: ({x}, {y})");
                }
            }
        }

        //

        {
            // Destructuring enums
            enum Message {
                Quit,
                Move { x: i32, y: i32 },
                Write(String),
                ChangeColor(i32, i32, i32),
            }

            let msg = Message::ChangeColor(0, 160, 255);

            match msg {
                Message::Quit => {
                    println!("The quit variant has no data to destructure")
                }
                Message::Move { x, y } => {
                    println!("Move in the x directon {x} and in the y directon {y}")
                }
                Message::Write(text) => {
                    println!("Text message : {text}");
                }
                Message::ChangeColor(r, g, b) => {
                    println!("Change the color to red {r}, green {g} and blue {b}");
                }
            }

            //
            //
            let numbers = (2, 43, 56, 74, 23);
            match numbers {
                (first, _, third, _, fifth) => {
                    println!("Some numbers: {first}, {third}, {fifth}")
                }
            }

            // Ignoring remaining parts of a vlue with ..
            {
                struct Point {
                    x: i32,
                    y: i32,
                    z: i32,
                }

                let origin = Point { x: 0, y: 0, z: 0 };

                // the syntax .. will expand to as many values as it needs to be.
                match origin {
                    Point { x, .. } => println!("x is {x}"),
                }
            }

            //

            // Extra conditionals with match guards
            //
            // When match guards are used, the compiler does not check for the exhaustiveness of
            // the match.
            {
                let num = Some(5);

                match num {
                    Some(x) if x % 2 == 0 => println!("The number {x} is even"),
                    Some(x) => println!("The number {x} is odd"),
                    None => (),
                };
                let x = 4;

                match x {
                    // matches if x is 4, 5 or 6. if the condition is false it prints "yes"
                    4 | 5 | 6 if false => println!("yes"),
                }
            }
            //
            {
                // @ Bindings
                //
                // The @ operator lets us create a variable that holds a value at the same time as
                // we're testing that value for a pattern match.
                enum Message {
                    Hello { id: i32 },
                }

                let msg = Message::Hello { id: 5 };

                match msg {
                    Message::Hello {
                        // we need to test that the id field is within the range 3..=7.
                        // we also want to bind the value to the variable id_variable so we can use
                        // it in the code bellow.
                        // The variable can also be named id, same as the field name.
                        id: id_variable @ 3..=7,
                    } => println!("Found an id in range : {id_variable}"),
                    Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
                    Message::Hello { id } => println!("Found some other id: {id}"),
                }
            }
        }
    }
}
