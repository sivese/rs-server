
struct Server{

}

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn handle_connection(mut stream : TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let content = fs::read_to_string("./svr/hel.html").unwrap();
        let res = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );

        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    else {
        // Do other something
    }
}

#[test]
fn server_test() {
    let lisner = TcpListener::bind("127.0.0.1:1818").unwrap();

    for stream in lisner.incoming() {
        let s = stream.unwrap();

        handle_connection(s);
    }
}