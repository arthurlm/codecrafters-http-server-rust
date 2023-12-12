use http_server_starter_rust::{request::HttpRequest, HttpHeader, HttpVerb};

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
