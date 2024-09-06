use std::{
    io::{self, Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    thread,
};

// Use the correct crate/module for HTTP request parsing
use simple_http::http::request;

fn create_socket() -> SocketAddr {
    SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5500)
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];

    // Read from the stream
    let bytes_read = stream.read(&mut buffer)?;

    // Convert buffer to string
    let buf_str = String::from_utf8_lossy(&buffer[..bytes_read]);

    // Parse the HTTP request
    let request = match request::HttpRequest::new(&buf_str) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed to parse request: {}", e);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Bad request"));
        }
    };

    // Generate an HTTP response
    let response = match request.response() {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Failed to generate response: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, "Internal server error"));
        }
    };

    // Prepare the full HTTP response with headers and body
    let response_str = response.to_string();

    // Print the response to the terminal
    println!("Sending response:\n{}", response_str);

    // Write the response to the stream
    stream.write_all(response_str.as_bytes())?;

    Ok(())
}

fn serve(socket: SocketAddr) -> io::Result<()> {
    let listener = TcpListener::bind(socket)?;
    let mut counter = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                counter += 1;
                println!("Connected stream.... {}", counter);

                // Spawn a new thread to handle the client connection
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
                continue;
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let socket = create_socket();
    serve(socket)?;
    Ok(())
}