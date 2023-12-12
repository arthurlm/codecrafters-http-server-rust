use std::io;

use crate::HttpStatusCode;

#[derive(Debug)]
pub struct HttpResponse {
    pub code: HttpStatusCode,
    pub content: HttpContent,
}

impl HttpResponse {
    pub fn new<C: Into<HttpContent>>(code: HttpStatusCode, content: C) -> Self {
        Self {
            code,
            content: content.into(),
        }
    }

    pub fn encode<W: io::Write>(&self, buf: &mut W) -> io::Result<()> {
        // Status line
        write!(
            buf,
            "HTTP/1.1 {} {}\r\n",
            self.code as u16,
            self.code.as_http_text()
        )?;

        // Headers
        if let Some(mime_type) = self.content.mime_type() {
            write!(buf, "Content-Type: {mime_type}\r\n")?;
        }
        if let Some(len) = self.content.content_len() {
            write!(buf, "Content-Length: {len}\r\n")?;
        }

        // Content
        write!(buf, "\r\n")?;
        self.content.encode(buf)?;

        Ok(())
    }

    pub fn to_string(&self) -> String {
        let mut buf = Vec::with_capacity(512);
        self.encode(&mut buf).expect("Fail to write string");
        String::from_utf8(buf).expect("Invalid UTF8 string")
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum HttpContent {
    NoContent,
    TextPlain(String),
    Bytes(Vec<u8>),
}

impl HttpContent {
    pub fn encode<W: io::Write>(&self, buf: &mut W) -> io::Result<()> {
        match self {
            HttpContent::NoContent => {}
            HttpContent::TextPlain(txt) => {
                write!(buf, "{txt}")?;
            }
            HttpContent::Bytes(data) => {
                buf.write_all(data)?;
            }
        }

        Ok(())
    }

    pub fn content_len(&self) -> Option<usize> {
        match self {
            HttpContent::NoContent => None,
            HttpContent::TextPlain(txt) => Some(txt.len()),
            HttpContent::Bytes(data) => Some(data.len()),
        }
    }

    pub fn mime_type(&self) -> Option<&'static str> {
        match self {
            HttpContent::NoContent => None,
            HttpContent::TextPlain(_) => Some("text/plain"),
            HttpContent::Bytes(_) => Some("application/octet-stream"),
        }
    }
}

impl From<String> for HttpContent {
    fn from(value: String) -> Self {
        Self::TextPlain(value)
    }
}

impl From<&str> for HttpContent {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<Vec<u8>> for HttpContent {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl From<&[u8]> for HttpContent {
    fn from(value: &[u8]) -> Self {
        value.to_vec().into()
    }
}

impl From<()> for HttpContent {
    fn from(_val: ()) -> Self {
        Self::NoContent
    }
}
