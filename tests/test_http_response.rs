use http_server_starter_rust::{response::*, HttpStatusCode};

#[test]
fn test_encode_basic_response() {
    let response = HttpResponse::new(HttpStatusCode::Ok, ());
    assert_eq!(response.to_string(), "HTTP/1.1 200 OK\r\n\r\n");
}

#[test]
fn test_encode_with_text_content() {
    let response = HttpResponse::new(HttpStatusCode::NotFound, "Hello world");
    assert_eq!(
        response.to_string(),
        "HTTP/1.1 404 Not Found\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: 11\r\n\
         \r\n\
         Hello world"
    );
}
