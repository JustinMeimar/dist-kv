use std::{net::TcpStream, io::{Write, Read}};
fn main() -> Result<(), std::io::Error>{

    let mut stream = TcpStream::connect("localhost:3000")?;

    let msg = "Test to tokio server";
    stream.write_all(msg.as_bytes())?;

    // let mut buf = String::new();
    // stream.read_to_string(&mut buf)?;

    // println!("recieved: {}", buf);

    Ok(())
}