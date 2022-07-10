use std::io::Read;
use std::net::{TcpListener, TcpStream};
use hmac::{Hmac, Mac};
use httparse::Header;
use serde_json::Value;
use sha2::Sha256;
use hex::encode;
use dotenv;
use std::io;

type HmacSha256 = Hmac<Sha256>;

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
    let header_size = req.parse(&buffer).unwrap().unwrap();

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
                    //println!("{} : {}", header.name, String::from_utf8_lossy(header.value));
                    let mut start_of_body: usize = 0;
                    while start_of_body < buffer.len() {
                        // TODO Make more robust check using carriage returns and line breaks.
                        if buffer[start_of_body] == '{' as u8 { break;} else { start_of_body += 1;}
                    }
                    let body = &buffer[start_of_body..actual_size];
                    //println!("Body: {} \n", String::from_utf8_lossy(&body));

                    let key = "password";
                    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("");
                    mac.update(body);

                    if String::from_utf8_lossy(&header.value[7..]) != encode(mac.finalize().into_bytes()) {
                        //return
                        // TODO Re-enable for actual webhooks
                    }

                    let body_json: Value = serde_json::from_slice(&body).expect("Cannot Parse JSON");

                    match dotenv::var("ACTION").unwrap().as_str() {
                        "DOCKER" => {
                            println!("Docker")
                        },
                        "GITHUB" => {
                            println!("Github")
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

