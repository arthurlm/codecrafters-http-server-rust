use std::{
    io::{self, Write},
    net::{TcpListener, TcpStream},
};

use http_server_starter_rust::response::{HttpResponse, HttpStatusCode};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(stream) {
                    eprintln!("Error while responding to client: {err}");
                }
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    println!("accepted new connection");
    let response = HttpResponse::new(HttpStatusCode::Ok);
    write!(stream, "{}", response.to_string())?;
    Ok(())
}
