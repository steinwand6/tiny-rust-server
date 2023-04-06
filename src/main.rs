use std::{
    error::Error,
    io::Read,
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_connection(stream);
        } else {
            println!("Something is faild.");
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    match stream.read(&mut buf) {
        Ok(_) => println!("Request: {}", String::from_utf8_lossy(&buf[..])),
        Err(e) => println!("{e}"),
    }
}
