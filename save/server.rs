use std::io::{self, BufReader, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::thread::{self, Thread};

fn handle_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &req_line[..] {
        "GET / HTTP/1.1"    => ("HTTP/1.1 200 OK", "public/ello.html"),
        _                   => ("HTTP/1.1 404 NOT FOUND", "public/404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let http_res = 
        format!("{}\r\nContent-Length: {}\r\n\r\n{}",status_line, length, content);  
    stream.write_all(http_res.as_bytes()).unwrap();
}

fn main() -> io::Result<()> {

    let listener = TcpListener::bind("localhost:8080")?;

    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream);
    }

    Ok(())
}