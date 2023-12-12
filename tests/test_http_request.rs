use http_server_starter_rust::{request::HttpRequest, HttpHeader, HttpServerError, HttpVerb};
use tokio::io::BufReader;

#[test]
fn test_parse() {
    let (_, req) = HttpRequest::parse(include_str!("data/req1.http")).unwrap();
    assert_eq!(
        req,
        HttpRequest {
            verb: HttpVerb::Get,
            target: "/".to_string(),
            headers: vec![],
        }
    );

    let (_, req) = HttpRequest::parse(include_str!("data/req2.http")).unwrap();
    assert_eq!(
        req,
        HttpRequest {
            verb: HttpVerb::Post,
            target: "/hello/world.html".to_string(),
            headers: vec![
                HttpHeader::new("host", "localhost:4221"),
                HttpHeader::new("user-agent", "curl/7.64.1")
            ],
        }
    );
}

#[test]
fn test_get_header() {
    let (_, req) = HttpRequest::parse(include_str!("data/req2.http")).unwrap();
    assert_eq!(req.get_header("HOST"), Some("localhost:4221"));
    assert_eq!(req.get_header("Host"), Some("localhost:4221"));
    assert_eq!(req.get_header("host"), Some("localhost:4221"));
    assert_eq!(req.get_header("user-agent"), Some("curl/7.64.1"));
    assert_eq!(req.get_header("whatever"), None);
}

#[tokio::test]
async fn test_parse_content_no_header() {
    let (remaining, req) = HttpRequest::parse(include_str!("data/req2.http")).unwrap();

    let content = req
        .read_content(&mut BufReader::new(remaining.as_bytes()))
        .await;

    assert_eq!(
        content,
        Err(HttpServerError::MissingHeader("content-length"))
    );
}

#[tokio::test]
async fn test_parse_content_bad_length() {
    let (remaining, req) = HttpRequest::parse(include_str!("data/req3.http")).unwrap();

    let content = req
        .read_content(&mut BufReader::new(remaining.as_bytes()))
        .await;

    assert_eq!(content, Err(HttpServerError::Io("early eof".to_string())));
}

#[tokio::test]
async fn test_parse_content_ok() {
    let (remaining, req) = HttpRequest::parse(include_str!("data/req4.http")).unwrap();

    let content = req
        .read_content(&mut BufReader::new(remaining.as_bytes()))
        .await;

    assert_eq!(content, Ok(b"hello world".to_vec()));
}
