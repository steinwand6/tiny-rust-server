use std::{error::Error, net::TcpListener};

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            println!("{:?}", stream);
        } else {
            println!("Something is faild.");
        }
    }
    Ok(())
}
