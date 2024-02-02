use futures::{
    io::Error,
    stream::StreamExt,
    task::{Context, Poll},
};

use async_std::io::{ReadExt, WriteExt};

// using an async runtime
#[async_std::main]
async fn main() {
    // Listen for incoming TCP connections on localhost port 7878
    let listener = async_std::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();

    listener
        .incoming()
        .for_each_concurrent(/*limit */ None, |tcpstream| async move {
            let tcpstream = tcpstream.unwrap();

            // async_std::task::spawn allows us to use multiple threads (parallelism)
            async_std::task::spawn(handle_connection(tcpstream));
        })
        .await;
    //
    // // Block forever, handling each request that arrives at this IP address
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //
    //     handle_connection(stream).await;
    // }
}

// async fn handle_connection(mut stream: async_std::net::TcpStream) {

/*
handle_connection() doesn't actually require an async_std::net::TcpStream; it requires any
struct that implements async_std::io::{Read, Write} and marker::Unpin
*/
async fn handle_connection(mut stream: impl async_std::io::Read + async_std::io::Write + Unpin) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "async_book/webserver/hello.html")
    } else if buffer.starts_with(sleep) {
        // async_std::task::sleep() is a non-blocking function unlike std::tread::sleep which
        // blocks
        async_std::task::sleep(std::time::Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "async_book/webserver/hello.html")
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            "async_book/webserver/404.html",
        )
    };
    let contents = std::fs::read_to_string(filename).unwrap();

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{status_line}{contents}");
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

struct MockTcpStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
}

impl async_std::io::Read for MockTcpStream {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        _: &mut Context,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        let size: usize = std::cmp::min(self.read_data.len(), buf.len());

        buf[..size].copy_from_slice(&self.read_data[..size]);

        return Poll::Ready(Ok(size));
    }
}
impl async_std::io::Write for MockTcpStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        _: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        self.write_data = Vec::from(buf);

        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _: &mut Context,
    ) -> Poll<Result<(), futures::io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: std::pin::Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }
}

impl Unpin for MockTcpStream {}

#[async_std::test]
async fn test_handle_connection() {
    let input_bytes = b"GET / HTTP/1.1\r\n";

    let mut contents = vec![0u8; 1024];

    contents[..input_bytes.len()].clone_from_slice(input_bytes);

    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
    };

    handle_connection(&mut stream).await;

    let expected_contents = std::fs::read_to_string("async_book/webserver/hello.html").unwrap();

    let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{expected_contents}");

    assert!(stream.write_data.starts_with(expected_response.as_bytes()));
}
