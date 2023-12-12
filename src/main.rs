use std::io;

use http_server_starter_rust::{
    request::HttpRequest, response::HttpResponse, HttpStatusCode, HttpVerb,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").await?;

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                tokio::spawn(handle_client(stream));
            }
            Err(err) => {
                eprintln!("error: {err}");
            }
        }
    }
}

async fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let req_raw = read_request_head(&mut stream).await?;

    let Ok((_, request)) = HttpRequest::parse(&req_raw) else {
        eprintln!("Invalid HTTP request: {req_raw:?}");
        return Ok(());
    };

    match (request.verb, request.target.as_str()) {
        (HttpVerb::Get, target) if target.starts_with("/echo/") => {
            let response = HttpResponse::new(HttpStatusCode::Ok, &target[6..]);
            stream.write_all(response.to_string().as_bytes()).await?;
            log_response(&request, &response);
        }
        (HttpVerb::Get, "/user-agent") => {
            let response = match request.get_header("user-agent") {
                Some(agent) => HttpResponse::new(HttpStatusCode::Ok, agent),
                None => HttpResponse::new(HttpStatusCode::BadRequest, ()),
            };
            stream.write_all(response.to_string().as_bytes()).await?;
            log_response(&request, &response);
        }
        (HttpVerb::Get, "/") => {
            let response = HttpResponse::new(HttpStatusCode::Ok, ());
            stream.write_all(response.to_string().as_bytes()).await?;
            log_response(&request, &response);
        }
        _ => {
            let response = HttpResponse::new(HttpStatusCode::NotFound, ());
            stream.write_all(response.to_string().as_bytes()).await?;
            log_response(&request, &response);
        }
    }

    Ok(())
}

async fn read_request_head<R: AsyncRead + Unpin>(stream: &mut R) -> io::Result<String> {
    let mut buf_reader = BufReader::new(stream);
    let mut output = String::with_capacity(1024);

    loop {
        let mut line = String::with_capacity(512);
        buf_reader.read_line(&mut line).await?;
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
