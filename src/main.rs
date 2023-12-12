use std::{env, io, path::PathBuf};

use http_server_starter_rust::{
    request::HttpRequest, response::HttpResponse, HttpStatusCode, HttpVerb,
};
use tokio::{
    fs,
    io::{AsyncBufReadExt, AsyncRead, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let root_dir = parse_cli_directory().unwrap_or_else(|| env::current_dir().unwrap());
    let listener = TcpListener::bind("127.0.0.1:4221").await?;
    println!("Serving files from: {root_dir:?}");

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                tokio::spawn(handle_client(stream, root_dir.clone()));
            }
            Err(err) => {
                eprintln!("error: {err}");
            }
        }
    }
}

fn parse_cli_directory() -> Option<PathBuf> {
    let index = env::args().position(|x| x == "--directory")?;
    let addr = env::args().nth(index + 1)?;
    addr.parse().ok()
}

async fn handle_client(mut stream: TcpStream, root_dir: PathBuf) -> io::Result<()> {
    let req_raw = read_request_head(&mut stream).await?;

    let Ok((_, request)) = HttpRequest::parse(&req_raw) else {
        eprintln!("Invalid HTTP request: {req_raw:?}");
        return Ok(());
    };

    let response = match (request.verb, request.target.as_str()) {
        (HttpVerb::Get, target) if target.starts_with("/echo/") => {
            HttpResponse::new(HttpStatusCode::Ok, &target[6..])
        }
        (HttpVerb::Get, target) if target.starts_with("/files/") => {
            match fs::read(root_dir.join(&target[7..])).await {
                Ok(content) => HttpResponse::new(HttpStatusCode::Ok, content),
                Err(_err) => HttpResponse::new(HttpStatusCode::NotFound, ()),
            }
        }
        (HttpVerb::Get, "/user-agent") => match request.get_header("user-agent") {
            Some(agent) => HttpResponse::new(HttpStatusCode::Ok, agent),
            None => HttpResponse::new(HttpStatusCode::BadRequest, ()),
        },
        (HttpVerb::Get, "/") => HttpResponse::new(HttpStatusCode::Ok, ()),
        _ => HttpResponse::new(HttpStatusCode::NotFound, ()),
    };

    stream.write_all(response.to_string().as_bytes()).await?;
    log_response(&request, &response);

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
