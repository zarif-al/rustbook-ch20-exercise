use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rustbook_ch20_exercise::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }

    println!("Shutting down server.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let binding = buf_reader.lines().next().unwrap().unwrap();

    let mut request_line = binding.split(" ");

    let request_method = request_line.next().unwrap();
    let request_route = request_line.next().unwrap();

    if request_method == "GET" {
        let (status_line, filename) = match request_route {
            "/" => ("HTTP/1.1 200 OK", "hello.html"),
            "/sleep" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        };

        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let response = format!("HTTP/1.1 404 NOT FOUND\r\n");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
