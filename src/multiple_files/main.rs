mod file1;
mod file2;
use adder;

fn main() {
    println!("multiple files");

    file1::file1::func_in_file1();

    file1::file1_func();

    let result = adder::add(5, 5);
    println!("result from adder is {result}");
}
