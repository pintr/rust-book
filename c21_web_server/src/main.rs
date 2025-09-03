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
    // single_threaded();
    multi_threaded();
}

fn _single_threaded() {
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
        for (i, stream) in listener.incoming().enumerate() {
            // The `incoming` method could generate errors when the clinet can't connect, because, in fact, the iteration is on connection attempts.
            // The connection attempt might fail for multiple reasons, such as OSs with a limited number of possible open connections.
            // When `stream` goes out of scope  and is dropped at the end of the loop, the onneciton is closed as part of the `drop` implementation.
            let stream = stream.unwrap();

            println!("Connection established!");
            println!("{:?}", stream);

            if i == 4 {
                // Limit to 5 connections to continue with the next experiments
                break;
            }
        }
        // Sometimes browsers deal with closed connections by retrying, and  they could open multiple connections to the same server
        // Each open connection is recognised as a different one, printing the info.
    }
    {
        // Reading the Request
        // Change the previous implementation to introduce the functionality of reading the request
        // The concern are separated by getting the connection, and doing tasks on the connections.
        // The function `handle_connection` is used to read data from the TCP stream and print it:
        use std::{
            io::{BufRead, BufReader}, // Import BufRead and BufReader to access traits and types used in the stream
            net::{TcpListener, TcpStream},
        };

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for (i, stream) in listener.incoming().enumerate() {
            let stream = stream.unwrap();

            handle_connection(stream); // Call `handle_connection` to work on the stream

            if i == 4 {
                // Limit to 5 connections to continue with the next experiments
                break;
            }
        }

        fn handle_connection(stream: TcpStream) {
            let buf_reader = BufReader::new(&stream); // Create a `BufReader` that wraps a reference to the `stream`, additionally it adds buffering
            let http_request: Vec<_> = buf_reader // `http_request` is used to collect the lines of the request, which are collected in a vector by adding the `Vec<_>` type
                .lines() // `BufReader` implements the `BufRead` trait which provides the `lines` method that returns an iterator on the stream of data  of type `Result`, which is splitted when there is a newline byte
                .map(|result| result.unwrap()) // To get each string is necessary to `map` and `unwrap` each result, in this version errors aren't handled
                .take_while(|line| !line.is_empty()) // The browser signals the end of a HTTP request by sending two newline chars in a row so, to get a request, lines are taken until an empty string is obtained
                .collect(); // The strings are collected into the vector to be printedusing the pretty debug format

            println!("Request: {http_request:#?}");
            // HTTP is a text based protocol, and a request takes the following format:
            // ```Method Request-URI HTTP-Version CRLF
            // headers CRLF
            // message-body```
            // The first line is the request line, that holds infomration about what a client is requesting
            // The first part is the method being used (e.g. `GET`)
            // The second part is the uniform resource identifier (URI) requested by the client
            // The last part is the HTTP version used by the client, and it ends in a CRLF (carriage return and line feed) sequence, it could be `\r` for the carriage return, and `\n` as line feed, it prints as a new line.
            // After this part, from `Host:` onward are headers, and `GET` requests have no body
        }
    }
    {
        // Writing a Response
        // A response to a HTTP request have this format:
        // ```HTTP-Version Status-Code Reason-Phrase CRLF
        // headers CRLF
        // message-body```
        // The first line contains the HTTP version used in the response, a numeric status that summarises the result of the request, and a text description of the status code
        // After the CRLF sequence there are the headers, another CRLF, and the body of the response, e.g. `HTTP/1.1 200 OK\r\n\r\n`
        // The status code 200 is the default success response, this is used in this version of the web server:
        use std::{
            io::{BufRead, BufReader, Write},
            net::{TcpListener, TcpStream},
        };

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for (i, stream) in listener.incoming().enumerate() {
            let stream = stream.unwrap();

            handle_connection(stream); // Call `handle_connection` to work on the stream

            if i == 4 {
                // Limit to 5 connections to continue with the next experiments
                break;
            }
        }

        fn handle_connection(mut stream: TcpStream) {
            let buf_reader = BufReader::new(&stream);
            let http_request: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            println!("Request: {http_request:#?}");

            let response = "HTTP/1.1 200 OK\r\n\r\n"; // Response variable containing the success message's data

            stream.write_all(response.as_bytes()).unwrap(); // The message is converted to bytes, and the `write_all` method on `stream` sends the bytes down the conection
            // Since `write_all` could failit requires the `unwrap()`, here used on any error
            // Now the `127.0.0.1:7878` page loads a blank page instead of an error
        }
    }
    {
        // Returning Real HTML
        // Now, instead of sending an empty response, a the minimal HTML content of `hello.html` will be sent
        use std::{
            fs,
            io::{BufRead, BufReader, Write},
            net::{TcpListener, TcpStream},
        };

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for (i, stream) in listener.incoming().enumerate() {
            let stream = stream.unwrap();

            handle_connection(stream); // Call `handle_connection` to work on the stream

            if i == 4 {
                // Limit to 5 connections to continue with the next experiments
                break;
            }
        }

        fn handle_connection(mut stream: TcpStream) {
            let buf_reader = BufReader::new(&stream);
            let http_request: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            println!("Request: {http_request:#?}");

            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("utils/hello.html").unwrap(); // Use `fs` to read the content of the `hello.html` file in utils
            let length = contents.len(); // To ensure a  valid HTTP response the `Content-Length` header is added to set the size of the length of the response body

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
            // Currently the request data is ignored and the content of the `hello.html` file is sent back unconditionally, meanning that any GET request on 127.0.0.1:7878 will get the same response
        }
    }
    {
        // Validating the Request and Selectively Responding
        // Right now the web server returns the HTML in the file `hello.html` unconditionally, let's check if the browser requests `/` before sending the HTML file, and return an error if something else is requested
        // To do this `handle_connection` method needs to check if the URI is correct:
        use std::{
            fs,
            io::{BufRead, BufReader, Write},
            net::{TcpListener, TcpStream},
        };

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for (i, stream) in listener.incoming().enumerate() {
            let stream = stream.unwrap();

            handle_connection(stream); // Call `handle_connection` to work on the stream

            if i == 4 {
                // Limit to 5 connections to continue with the next experiments
                break;
            }
        }

        fn handle_connection(mut stream: TcpStream) {
            let buf_reader = BufReader::new(&stream);
            let request_line = buf_reader.lines().next().unwrap().unwrap(); // Look at the first line of the response to check the URI
            // The first `unwrap` takes care of the `Option`, while the second `unwrap` handles the `Result`, and has the same effect as the `unwrap` in the `map`

            if request_line == "GET / HTTP/1.1" {
                // Check if the `request_line` equals the request line of a GET to the `/` path, if it does return the `hello.html` file content
                let status_line = "HTTP/1.1 200 OK";
                let contents = fs::read_to_string("utils/hello.html").unwrap();
                let length = contents.len();

                let response =
                    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

                stream.write_all(response.as_bytes()).unwrap();
            } else {
                // If the request line differs from `/` return a 404 response with a the `404.html` file content referring to an unexisting resource (code 404)
                let status_line = "HTTP/1.1 404 NOT FOUND";
                let contents = fs::read_to_string("utils/404.html").unwrap();
                let length = contents.len();

                let response =
                    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

                stream.write_all(response.as_bytes()).unwrap();
            }
            // Now the program checks whether a user request for the `/` path and returns the content, otherwise it returns a 404 error page.
        }
    }
    {
        // A Touch of Refactoring
        // In the previous version the `if` and `else` blocks have a lot of repetition, since they both read a file, and write the content to the stream, the only difference is in the status line and the filename.
        // The code can be improved by pulling out the differences into separate `if` and `else` blocks that will assign the values of status line and filename, using the variables unconditionally, replacing the larger blocks.
        use std::{
            fs,
            io::{BufRead, BufReader, Write},
            net::{TcpListener, TcpStream},
        };

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for (i, stream) in listener.incoming().enumerate() {
            let stream = stream.unwrap();

            handle_connection(stream); // Call `handle_connection` to work on the stream

            if i == 4 {
                // Limit to 5 connections to continue with the next experiments
                break;
            }
        }

        fn handle_connection(mut stream: TcpStream) {
            let buf_reader = BufReader::new(&stream);
            let request_line = buf_reader.lines().next().unwrap().unwrap();

            let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
                ("HTTP/1.1 200 OK", "utils/hello.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND", "utils/404.html")
            };
            // The assignment of `status_line` and `filename` is destructured and added to a tuple so there is no duplicated code

            let contents = fs::read_to_string(filename).unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn multi_threaded() {
    // Currently the server processes each request in turn, meaning that it won't process a second request until the first is finished.
    // This serial execution wou ld be less and less optimal when multiple requests are received, in particular if they are long.
    {
        // Simulating a Slow Request in the Current Server Implementation
        // Here is the single threaded server with an additional URI `/sleep` that simulates a slow response
        use std::{
            fs,
            io::{BufRead, BufReader, Write},
            net::{TcpListener, TcpStream},
            thread,
            time::Duration,
        };

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for (i, stream) in listener.incoming().enumerate() {
            let stream = stream.unwrap();
            handle_connection(stream);

            if i == 9 {
                // Limit to 10 connections to continue with the next experiments
                break;
            }
        }

        fn handle_connection(mut stream: TcpStream) {
            let buf_reader = BufReader::new(&stream);
            let request_line = buf_reader.lines().next().unwrap().unwrap();

            let (status_line, filename) = match &request_line[..] {
                // Switch from `if` to `match` since there are more than two cases
                // This requires to match on a slice of `request_line` becuase it doesn't do automatic referencing and dereferencing
                "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "utils/hello.html"),
                "GET /sleep HTTP/1.1" => {
                    thread::sleep(Duration::from_secs(5)); // Wait 5 second before sending the response

                    ("HTTP/1.1 200 OK", "utils/hello.html")
                }
                _ => ("HTTP/1.1 404 NOT FOUND", "utils/404.html"),
            };

            let contents = fs::read_to_string(filename).unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nCOntent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();

            // Trying to load `/sleep` and then `/` the first request requires 5 seconds, the second one rquires the first to finish (so 5 seconds + time to respond)
            // This can be avoided with multiple techniques, including using async and a thread pool
        }
    }
    {
        // Improving Throughput with a Thread Pool
        // A thread pool is a group of spawned threads that are waiting and ready to handle a task.
        // When the program reaceives a new task, it assigns one of the threads of the pool to that task, and that thread will process it.
        // The remaining threads in the pool are available to handle any other task.
        // When the first thread is done processing its task, it returns to the pool of idle threads, reasy to handle a new task.
        // A thread pool allows to process connections concurrently, increasing the throughput of the server.
        // In this example the number of threads will be limited to afixed number, to avoid DoS attacks.
        // The requests that com in are sento to the pool for processing, the pool will maintain a queue.
        // Each thread will pop off a request from this queue, handle the request, and then ask the queue for another request
        // In this way, even with long running requests, the server is able to handle multiple requests concurrently
        // This technique is one of many ways to improve thoughput of a web server.
        // Other options include fork/join model, singlt-threaded async I7O model, and the multi-threaded async I/O model.
        // Before implementing a thread pool it is important to write the clinet interface to guide the design, so write the APIs, and implement the functionalities.
        // Here compiler-driven developmenmt is used, so first the functions will be written, and look at the compiler's errors to determine how to change the code to work.
        // In this examples the use declarations and the `handle_connection` function will remain the same as before, so they will be reused for each version

        use std::{
            fs,
            io::{BufRead, BufReader, Write},
            net::{TcpListener, TcpStream},
            thread,
            time::Duration,
        };

        fn handle_connection(mut stream: TcpStream) {
            let buf_reader = BufReader::new(&stream);
            let request_line = buf_reader.lines().next().unwrap().unwrap();

            let (status_line, filename) = match &request_line[..] {
                // Switch from `if` to `match` since there are more than two cases
                // This requires to match on a slice of `request_line` becuase it doesn't do automatic referencing and dereferencing
                "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "utils/hello.html"),
                "GET /sleep HTTP/1.1" => {
                    thread::sleep(Duration::from_secs(5)); // Wait 5 second before sending the response

                    ("HTTP/1.1 200 OK", "utils/hello.html")
                }
                _ => ("HTTP/1.1 404 NOT FOUND", "utils/404.html"),
            };

            let contents = fs::read_to_string(filename).unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nCOntent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();

            // Trying to load `/sleep` and then `/` the first request requires 5 seconds, the second one rquires the first to finish (so 5 seconds + time to respond)
            // This can be avoided with multiple techniques, including using async and a thread pool
        }

        {
            // Spawning a Thread for each Request
            // This example creates a new thread for every connection.
            // This isn't the final version because it's vulnerabel to DoS when an unlimited numebr of threads is spawned, but it's a starting point to a multithread web server.
            // The next examples will rely on a thread pool
        }
    }
}
