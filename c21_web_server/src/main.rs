//! This is the final chapter of the book: a project of a web server that prints "Hello, Hi from Rust" in a web browser.
//! The plan is:
//! 1. Dig into TCP and HTTP.
//! 2. Listen for TCP connections on a socket.
//! 3. parse a small number of HTTP requests.
//! 4. Create a proper HTTP response.
//! 5. improve the throughput of the server with a thread pool.
//! This is a small example of web server with thread pool, not the best available for a web server and thread pool.
//! In the project async and await won't be used in order to keep it simple, without adding an async runtime.

fn main() {
    single_threaded();
}

fn single_threaded() {
    // In order to build a single threaded web server it is necessary to have an overview of the protocols involved
    // The main request-response protocols are TCP and HTTP:
    // - TCP: Describe the details of how information gets form one server to another
    // - HTTP: Built on top of TCP by defining the contents of the requests and responses
    {
        // Listening to the TCP Connection
        // The web server needs to listen to a TCP connection using the `std::net` module.
        // In this case the code will listen to `127.0.0.1:7878` for incoming TCP streams.
        // Once the server receives an incoming stream, it prints "Connection established!" and the info of the peer:
        use std::net::TcpListener;

        // `TcpListener` is used to TCP connecitons on `127.0.0.1:7878`, so localhost on the port 7878
        // 7878 is chosen because it's not a common port for HTTP (80, and 8080), and because is Rust with T9
        // The `bind` function returns a new `TcpListener` instance
        // It's called `bind` because connecting to a port to listen to is known as "binding to a port".
        // The `bind` function returns a `Result<T, E>`, because the binding could fail
        // Examples of failures could be connecting to port unde 1023 as non admin, or running two instances listening on the same port.
        // Usually the error should be handled, since it's a simple server it will rely on unwrap and if there is an error, the program stops.
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        // The `incoming` method on `TcpListener` returns an iterator giving a sequence of streams of type `TcpStream`
        // A stream is an open connection between client and server, and a connection is the full request and response process.
        // `TcpStream` is used to to read the client request and write the server response. The loop will process each connection in turn.
        for stream in listener.incoming() {
            // The `incoming` method could generate errors when the clinet can't connect, because, in fact, the iteration is on connection attempts.
            // The connection attempt might fail for multiple reasons, such as OSs with a limited number of possible open connections.
            // When `stream` goes out of scope  and is dropped at the end of the loop, the onneciton is closed as part of the `drop` implementation.
            let stream = stream.unwrap();

            println!("Connection established!");
            println!("{:?}", stream);
        }
        // Sometimes browsers deal with closed connections by retrying, and  they could open multiple connections to the same server
        // Each open connection is recognised as a different one, printing the info.
    }
    {
        // Reading the Request
        // Change the previous implementation to introduce the funcitonality of reading the request
        // The concern are separated by getting the connection, and doing tasks on the connections.
        // The function `handle_connection` is used to read data from the TCP stream and print it:
        use std::{
            io::{BufReader, prelude::*},
            net::{TcpListener, TcpStream},
        };

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream);
        }

        fn handle_connection(stream: TcpStream) {
            let buf_reader = BufReader::new(&stream);
            let http_request: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            println!("Request: {http_request:#?}");
        }
    }
}
