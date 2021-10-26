use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    println!("Starting TCP server...");
    
    // create listener, bind to socket
    // print message on error
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Unable to bind to socket");
    
    // keep listening, handle error
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(error) => panic!("Error: {}", error),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Connection established!");

    // set a buffer of 1024 bytes
    let mut buffer = [0; 1024];
    
    // keep reading 
    loop {
        // read method returns a size (how many bytes were read)
        match stream.read(&mut buffer) {
            Ok(size) => {
                stream.write(&buffer[0..size]).unwrap();
            },
            Err(error) => {
                println!("Disconnected! Error: {}", error);
                // stop listen on error
                break;
            }
        }
    }
}
