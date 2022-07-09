use std::collections::HashMap;
use std::hash::Hash;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use serde_json::Value;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8765").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_conn(stream);
    }

    println!("Hello, world!");
}

fn handle_conn(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    //println!("{}", String::from_utf8_lossy(&buffer));

    let data: HashMap<String, Value> = serde_json::from_slice(parse_data(&buffer)).expect("");
    let headers: HashMap<String, String> = parse_headers(&buffer);

    for (key, value) in data {
        println!("{} : {}", key, value);
    }


}

fn parse_data(buffer: &[u8]) -> &[u8] {
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut index: usize = 0;
    let mut count: usize = 0;
    for i in buffer { // Parse out the json from the request
        if *i == '{' as u8 {
            count += 1;
            if start == 0 {
                start = index;
            }
        } else if *i == '}' as u8 {
            count -= 1;
            if count == 0 {
                end = index;
                break;
            }
        };
        index += 1;
    }
    start -= 1; // Include the json braces
    end += 1;

    &buffer[start..end]
}

fn parse_headers(buffer: &[u8]) -> HashMap<String, String> {
    // Parses the headers from the request,
    // If there is an error, throw out the request.

    let mut headers: HashMap<String, String> = HashMap::new();

    let mut i: usize = 0;
    while i < buffer.len() {

        if buffer[i] == 0x0A  { // 0x0A is newline
            if buffer[i+1] == 0x0A {
                println!("{}", i);
                println!("{}", String::from_utf8_lossy(&buffer[0..i]));
                break;
            }
        }
        i += 1;
    }

    headers
}
