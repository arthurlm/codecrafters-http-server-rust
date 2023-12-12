use std::{
    io::{self, BufRead, BufReader, Read, Write},
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
    let req_raw = read_request_head(&mut stream)?;

    let Ok((_, request)) = HttpRequest::parse(&req_raw) else {
        eprintln!("Invalid HTTP request: {req_raw:?}");
        return Ok(());
    };

    match (request.verb, request.target.as_str()) {
        (HttpVerb::Get, target) if target.starts_with("/echo/") => {
            let response = HttpResponse::new(HttpStatusCode::Ok, &target[6..]);
            write!(stream, "{}", response.to_string())?;
            log_response(&request, &response);
        }
        (HttpVerb::Get, "/") => {
            let response = HttpResponse::new(HttpStatusCode::Ok, ());
            write!(stream, "{}", response.to_string())?;
            log_response(&request, &response);
        }
        _ => {
            let response = HttpResponse::new(HttpStatusCode::NotFound, ());
            write!(stream, "{}", response.to_string())?;
            log_response(&request, &response);
        }
    }

    Ok(())
}

fn read_request_head<R: Read>(stream: &mut R) -> io::Result<String> {
    let mut buf_reader = BufReader::new(stream);
    let mut output = String::with_capacity(1024);

    loop {
        let mut line = String::with_capacity(512);
        buf_reader.read_line(&mut line)?;
        output.push_str(&line);
        if line == "\r\n" {
            break;
        };
    }

    Ok(output)
}

fn log_response(req: &HttpRequest, res: &HttpResponse) {
    println!(
        "Response: {:?} '{}' => {:?}",
        req.verb, req.target, res.code
    );
}
