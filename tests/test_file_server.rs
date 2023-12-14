use http_server_starter_rust::{
    file_server::{save_file, serve_file},
    response::HttpResponse,
    HttpStatusCode,
};
use tokio::fs;

#[tokio::test]
async fn test_serve_file() {
    // Invalid
    let response = serve_file("invalid path").await;
    assert_eq!(response, HttpResponse::new(HttpStatusCode::NotFound, ()));

    // Valid
    let response = serve_file("tests/data/payload.txt").await;
    assert_eq!(
        response,
        HttpResponse::new(HttpStatusCode::Ok, b"Hello world !".as_slice())
    );
}

#[tokio::test]
async fn test_save_file() {
    // Invalid
    let response = save_file("/invalid/path.txt", b"whatever").await;
    assert_eq!(
        response,
        HttpResponse::new(HttpStatusCode::InternalServerError, ())
    );

    // Invalid
    let path = "/tmp/text.txt";
    let content = "good content";

    let response = save_file(path, content.as_bytes()).await;
    assert_eq!(response, HttpResponse::new(HttpStatusCode::Created, ()));
    assert_eq!(fs::read_to_string(path).await.unwrap(), content);
}
