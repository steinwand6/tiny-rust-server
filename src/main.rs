use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
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

    let file_name = "hello.html";

    let mut contents = String::new();
    if let Err(e) = File::open(file_name).and_then(|mut file| file.read_to_string(&mut contents)) {
        eprintln!("Error with {file_name}: {e}");
        contents = "
			<head><title>Error!</title></head>
			<body><h1>Error!</h1></body>"
            .to_string();
    }

    let res_header = "HTTP/1.1 200 OK\r\n\r\n";
    match stream.read(&mut buf) {
        Ok(_) => {
            println!("Request: {}", String::from_utf8_lossy(&buf[..]));
            let response = format!("{res_header}{contents}");
            if let Err(e) = stream.write(response.as_bytes()) {
                eprintln!("Error writing to stream: {e}");
            }
            if let Err(e) = stream.flush() {
                eprintln!("Error flush stream: {e}");
            }
        }
        Err(e) => {
            eprintln!("Error reading from stream {e}");
        }
    }
}
