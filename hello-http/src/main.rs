use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const HOME_CONTENT: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
"#;

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("localhost:8082").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection established");
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 ok";
    let len = HOME_CONTENT.len();
    stream
        .write_all(
            format!("{status_line}\r\nContent-Length: {len}\r\n\n\r{HOME_CONTENT}").as_bytes(),
        )
        .unwrap();
    println!("Request: {:#?}", http_request);
}
