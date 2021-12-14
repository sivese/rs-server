
struct Server{

}

use crate::thread::pool::ThreadPool;

use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn handle_connection(mut stream : TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, file) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    }
    else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string("404.html").unwrap();

    let res = format!(
        "{}\r\nCotent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}

#[test]
fn server_test() {
    let lisner = TcpListener::bind("127.0.0.1:1818").unwrap();
    let pool = ThreadPool::new(4);

    for stream in lisner.incoming() {
        let s = stream.unwrap();
        
        handle_connection(s);
    }
}