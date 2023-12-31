#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[repr(u16)]
#[non_exhaustive]
pub enum HttpStatusCode {
    #[default]
    Ok = 200,
    Created = 201,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl HttpStatusCode {
    pub fn as_http_text(&self) -> &'static str {
        match self {
            HttpStatusCode::Ok => "OK",
            HttpStatusCode::Created => "Created",
            HttpStatusCode::BadRequest => "Bad Request",
            HttpStatusCode::NotFound => "Not Found",
            HttpStatusCode::InternalServerError => "Internal Server Error",
        }
    }
}
