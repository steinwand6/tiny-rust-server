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
            eprintln!("Something is faild.");
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    match stream.read(&mut buf) {
        Ok(_) => {
            println!("Request: {}", String::from_utf8_lossy(&buf[..]));

            let mut headers = [httparse::EMPTY_HEADER; 64];
            let mut req = httparse::Request::new(&mut headers);
            if let Err(e) = req.parse(&mut buf) {
                eprintln!("Error parsing request: {e}");
                let response = build_response(400, "400.html");
                send_response(stream, response);
                return;
            };

            let response;
            match req.method {
                Some("GET") => response = handle_get_request(&req),
                _ => return,
            }

            send_response(stream, response);
        }
        Err(e) => {
            eprintln!("Error reading from stream {e}");
        }
    }
}

fn handle_get_request(req: &httparse::Request) -> String {
    match req.path {
        Some("/") => {
            let file_name = "hello.html";
            build_response(200, file_name)
        }
        _ => {
            let file_name = "404.html";
            build_response(404, file_name)
        }
    }
}

fn build_response(status_code: u16, file_name: &str) -> String {
    let status_message = get_status_message_for_code(status_code);
    let contents = read_file(file_name).unwrap_or_else(|e| {
        eprintln!("Error with {file_name}: {e}");
        return get_status_message_for_code(500);
    });
    format!("{status_message}\r\n\r\n{contents}")
}

fn send_response(mut stream: TcpStream, res: String) {
    if let Err(e) = stream.write(res.as_bytes()) {
        eprintln!("Error writing to stream: {e}");
    }
    if let Err(e) = stream.flush() {
        eprintln!("Error flush stream: {e}");
    }
}

fn get_status_message_for_code(status_code: u16) -> String {
    match status_code {
        200 => format!("HTTP/1.1 200 OK"),
        404 => format!("HTTP/1.1 404 Not Found"),
        500 => format!("HTTP/1.1 500 Internal ServerError"),
        _ => unreachable!(),
    }
}

fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    let mut contents = String::new();

    File::open(file_name).and_then(|mut file| file.read_to_string(&mut contents))?;
    Ok(contents)
}
