use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use http_server_starter_rust::{
    request::HttpRequest, response::HttpResponse, HttpStatusCode, HttpVerb,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(stream) {
                    eprintln!("I/O error while responding to client: {err}");
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

    let mut buf_reader = BufReader::new(stream.try_clone()?);
    let mut req_raw = String::with_capacity(512);
    buf_reader.read_line(&mut req_raw)?;

    let Ok((_, request)) = HttpRequest::parse(&req_raw) else {
        eprintln!("Invalid HTTP request: {req_raw}");
        return Ok(());
    };

    match (request.verb, request.target.as_str()) {
        (HttpVerb::Get, target) if target.starts_with("/echo/") => {
            let response = HttpResponse::new(HttpStatusCode::Ok, &target[6..]);
            write!(stream, "{}", response.to_string())?;
        }
        (HttpVerb::Get, "/") => {
            let response = HttpResponse::new(HttpStatusCode::Ok, ());
            write!(stream, "{}", response.to_string())?;
        }
        _ => {
            let response = HttpResponse::new(HttpStatusCode::NotFound, ());
            write!(stream, "{}", response.to_string())?;
        }
    }

    Ok(())
}
