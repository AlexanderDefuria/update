use std::io::Read;
use std::net::{TcpListener, TcpStream};
use httparse::Header;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8765").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_conn(stream);
    }

    println!("Hello, world!");
}

fn handle_conn(mut stream: TcpStream) {
    let mut buffer = [0; 1024*64]; // 64kb
    let actual_size = stream.read(&mut buffer).unwrap();

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    req.parse(&buffer).unwrap();

    if !headers.is_empty() {
        for header in headers {
            match header {
                Header { name: "X-GitHub-Event", .. } => {
                    println!("{} : {}", header.name, String::from_utf8_lossy(header.value));
                }
                Header { name: "X-GitHub-Delivery", .. } => {
                    println!("{} : {}", header.name, String::from_utf8_lossy(header.value));
                }
                Header { name: "X-Hub-Signature-256", .. } => {
                    println!("{} : {}", header.name, String::from_utf8_lossy(header.value));
                    let mut start_of_body: usize = 0;
                    while start_of_body < buffer.len() {
                        // TODO Make more robust check using carriage returns and line breaks.
                        if buffer[start_of_body] == '{' as u8 { break;} else { start_of_body += 1;}
                    }
                    let body = &buffer[start_of_body..actual_size];
                    println!("{}", String::from_utf8_lossy(&body));
                }
                _ => {}
            }
        }
    }


}
