use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    // single_threaded();
    multi_threaded();
}

fn _single_threaded() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

mod thread_pool;
use thread_pool::thread_pool::ThreadPool;
// Improving Throughput with Thread pool
//
// A thread pool is a grouop of spawned threads that are waiting and ready to handle a task.
fn multi_threaded() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    // // modify listener to only take 2 requests inorder to test graceful shutdown.
    // // The take method is defined in the Iterator trait and limits the Iteration to the first
    // two items at most.
    // for stream in listener.incoming().take(2) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("--- stream received ---");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("shutting down")
}

fn handle_connection(mut stream: TcpStream) {
    // create a new Bufreader instance that wraps a mutable reference to the stream. BufReader adds
    // buffering by managing calls to the std::io::Read trait methods for us.
    let buf_reader = BufReader::new(&mut stream);

    // the first unwrap takes care of the Option from next and stops if the iterator has no items.
    // the second unwrap handles the Result.
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {}", request_line);

    // we need to explicitly match on a slice or request_line.
    // Match doesn't do automatic referencing and dereferencing like the equality method does.
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => (
            "HTTP/1.1 200 OK",
            "rust_programming/src/webserver/hello.html",
        ),
        "GET /sleep HTTP/1.1" => {
            // simulate a slow thread with sleep timer
            std::thread::sleep(std::time::Duration::from_secs(10));
            (
                "HTTP/1.1 200 OK",
                "rust_programming/src/webserver/hello.html",
            )
        }
        _ => (
            "HTTP/1.1 404 NOT FOUND",
            "rust_programming/src/webserver/404.html",
        ),
    };

    let contents = fs::read_to_string(filename).unwrap();

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    println!("Response: {status_line}");
}
