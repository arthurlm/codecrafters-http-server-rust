use http_server_starter_rust::{response::*, HttpStatusCode};

#[test]
fn test_encode_basic_response() {
    let response = HttpResponse::new(HttpStatusCode::Ok);
    assert_eq!(response.to_string(), "HTTP/1.1 200 OK\r\n\r\n");
}
