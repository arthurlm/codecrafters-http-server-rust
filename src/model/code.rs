#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[repr(u16)]
#[non_exhaustive]
pub enum HttpStatusCode {
    #[default]
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl HttpStatusCode {
    pub fn as_http_text(&self) -> &'static str {
        match self {
            HttpStatusCode::BadRequest => "Bad Request",
            HttpStatusCode::Ok => "OK",
            HttpStatusCode::NotFound => "Not Found",
        }
    }
}
