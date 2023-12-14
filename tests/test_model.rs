use http_server_starter_rust::{HttpHeader, HttpStatusCode, HttpVerb};

#[test]
fn test_http_status_code() {
    assert_eq!(format!("{:?}", HttpStatusCode::Created), "Created");
    assert_eq!(HttpStatusCode::NotFound.clone(), HttpStatusCode::NotFound);
    assert_eq!(*&HttpStatusCode::NotFound, HttpStatusCode::NotFound);
    assert_eq!(HttpStatusCode::default() as u16, 200);
    assert_eq!(HttpStatusCode::Ok.as_http_text(), "OK");
    assert_eq!(HttpStatusCode::Created.as_http_text(), "Created");
    assert_eq!(HttpStatusCode::BadRequest.as_http_text(), "Bad Request");
    assert_eq!(HttpStatusCode::NotFound.as_http_text(), "Not Found");
    assert_eq!(
        HttpStatusCode::InternalServerError.as_http_text(),
        "Internal Server Error"
    );
}

#[test]
fn test_header() {
    // New
    let (_, h2) = HttpHeader::parse("AUTH: Bearer XXXXX\r\n").unwrap();

    // Parse
    assert!(HttpHeader::parse("Accept").is_err());
    assert!(HttpHeader::parse("Accept:").is_err());
    assert!(HttpHeader::parse("Accept: application/json").is_err());
    assert!(HttpHeader::parse("Accept: application/json\r\n").is_ok());

    let h1 = HttpHeader::new("Auth", "Bearer XXXXX");

    // Derive
    assert_eq!(
        format!("{h1:?}"),
        r#"HttpHeader { name: "auth", value: "Bearer XXXXX" }"#
    );
    assert_eq!(h1.clone(), h1);
    assert_eq!(h1, h2);
}

#[test]
fn test_http_verb() {
    // Derive
    assert_eq!(format!("{:?}", HttpVerb::Get), "Get");
    assert_eq!(HttpVerb::Get.clone(), HttpVerb::Get);
    assert_eq!(*&HttpVerb::Get, HttpVerb::Get);

    // Parse
    macro_rules! assert_parse_eq {
        ($v:expr, $r:expr) => {
            let (_, v) = HttpVerb::parse($v).unwrap();
            assert_eq!(v, $r);
        };
    }

    assert!(HttpVerb::parse("Yeah").is_err());
    assert_parse_eq!("HEAD", HttpVerb::Head);
    assert_parse_eq!("OPTIONS", HttpVerb::Options);
    assert_parse_eq!("GET", HttpVerb::Get);
    assert_parse_eq!("PUT", HttpVerb::Put);
    assert_parse_eq!("POST", HttpVerb::Post);
    assert_parse_eq!("DELETE", HttpVerb::Delete);

    // As HTTP text
    assert_eq!(HttpVerb::Head.as_http_text(), "HEAD");
    assert_eq!(HttpVerb::Options.as_http_text(), "OPTIONS");
    assert_eq!(HttpVerb::Get.as_http_text(), "GET");
    assert_eq!(HttpVerb::Put.as_http_text(), "PUT");
    assert_eq!(HttpVerb::Post.as_http_text(), "POST");
    assert_eq!(HttpVerb::Delete.as_http_text(), "DELETE");
}
