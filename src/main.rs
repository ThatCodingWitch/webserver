
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

    let mime_type = new_mime_guess::from_path(filepath.clone()).first_or_octet_stream();
    println!("{:?}", mime_type);

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



