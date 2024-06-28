mod mime_types;

use webserver::ThreadPool;
use std::{
    fs,
    io::prelude::*,
    net::TcpListener,
    net::TcpStream,
};
use std::io::BufReader;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    println!("\nrequest");
    let buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => {
            eprintln!("Failed to read line from stream: {}", e);
            return;
        },
        None => {
            eprintln!("Stream is empty");
            eprintln!("{:?}", stream);
            return;
        }
    };

    println!("{}", request_line);


    let mut filepath = request_line.split(' ').nth(1).unwrap().to_string();
    if filepath == "/"   {
        filepath = "/index.html".parse().unwrap()
    }
    let filetype = filepath.split('.').last().unwrap().to_string();
    let mime_types = mime_types::get_mime_types();
    let mime_type = match mime_types.get(&filetype.as_str()) {
        None => "application/octet-stream",
        Some(mime) => mime,
    };

    filepath = format!("./front-end{}", filepath);
    let (contents, status_line) = match fs::read(&filepath) {
        Ok(contents) => (contents, "HTTP/1.1 200".to_string()),
        Err(_) => {
            filepath = "./built-in/404.html".to_string();
            match fs::read(&filepath) {
                Ok(contents) => (contents, "HTTP/1.1 404".to_string()),
                Err(_) => (Vec::new(), "HTTP/1.1 500".to_string())
            }
        }
    };

    let length = contents.len();
    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        status_line, mime_type, length
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap();
}



