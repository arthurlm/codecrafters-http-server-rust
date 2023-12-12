use http_server_starter_rust::{request::HttpRequest, HttpVerb};

#[test]
fn test_parse() {
    let (_, req) = HttpRequest::parse(include_str!("data/req1.txt")).unwrap();
    assert_eq!(
        req,
        HttpRequest {
            verb: HttpVerb::Get,
            target: "/index.html".to_string(),
        }
    );
}
