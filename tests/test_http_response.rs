use http_server_starter_rust::{response::*, HttpStatusCode};

#[test]
fn test_encode_basic_response() {
    let response = HttpResponse::new(HttpStatusCode::Ok, ());
    assert_eq!(response.to_http_string(), "HTTP/1.1 200 OK\r\n\r\n");
}

#[test]
fn test_encode_with_text_content() {
    let response = HttpResponse::new(HttpStatusCode::NotFound, "Hello world");
    assert_eq!(
        response.to_http_string(),
        "HTTP/1.1 404 Not Found\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: 11\r\n\
         \r\n\
         Hello world"
    );
}

#[test]
fn test_encode_with_binary_content() {
    let response = HttpResponse::new(HttpStatusCode::NotFound, [65, 66, 67].as_slice());
    assert_eq!(
        response.to_http_string(),
        "HTTP/1.1 404 Not Found\r\n\
         Content-Type: application/octet-stream\r\n\
         Content-Length: 3\r\n\
         \r\n\
         ABC"
    );
}
