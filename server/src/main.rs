use std::fs;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

// 1. set tcp listener to establish connections and listen to the TCP connection
// 2. reading data from TCP stream
// 3. response with the following format
//      HTTP-Version Status-Code Reason-Phrase CRLF
//      headers CRLF
//      message-body
//
//      example: HTTP/1.1 200 OK\r\n\r\n
// 4. Returning valid html - create index.html
//unwrap it returns Ok or Some or it panics with a default error massage if it's Err or None
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connections established!");
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    // create a buffer to hold a data that is red
    let mut buffer = [0; 1024]; //1024 bites stream long

    // check if reading data from a stream
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
