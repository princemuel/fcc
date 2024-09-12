use server::ThreadPool;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

// http://127.0.0.1:7878
fn main() {
    let addr = "127.0.0.1:7878";
    let listener = TcpListener::bind(addr).unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

// * HTTP-Version Status-Code Reason-Phrase CRLF
// * Headers CRLF
// * Message Body
//
// * example: HTTP/1.1 200 OK\r\n\r\n
fn handle_connection(mut stream: TcpStream) {
    // let mut buffer = [0; 1024];

    // stream.read(&mut buffer).unwrap();

    // let get = b"GET / HTTP/1.1\r\n";
    // let sleep = b"GET /sleep HTTP/1.1\r\n";

    // let (status_line, filename) = if buffer.starts_with(get) {
    //     ("HTTP/1.1 200 OK", "index.html")
    // } else if buffer.starts_with(sleep) {
    //     thread::sleep(Duration::from_secs(5));
    //     ("HTTP/1.1 200 OK", "sleep.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        | "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        | "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        | _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
