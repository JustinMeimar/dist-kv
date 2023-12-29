use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, BufWriter};
use tokio::io::BufReader;
use tokio::time::{self, Duration};
use std::fs;

async fn handle_connection(mut socket: TcpStream) {
    let (reader, writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    if let Ok(Some(req_line)) = reader.lines().next_line().await {
        let (status_line, filename) = match req_line.as_str() {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "public/ello.html"),
            _ => ("HTTP/1.1 404 NOT FOUND", "public/404.html"),
        };

        println!("status_line: {}", status_line);

        if let Ok(content) = fs::read_to_string(filename) {            
            let http_response = format!(
                "{}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                status_line, content.len(), content
            );

            writer.write_all(http_response.as_bytes()).await.unwrap();
            writer.flush().await.unwrap();  // Ensure buffer is flushed
        }

    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("localhost:3000").await?;

    loop {
        if let Ok((socket, _)) = listener.accept().await {
            tokio::spawn(async move {
                handle_connection(socket).await;
            });
        }
    }
}