mod code;

use std::io;

pub use code::HttpStatusCode;

#[derive(Debug)]
pub struct HttpResponse {
    pub code: HttpStatusCode,
}

impl HttpResponse {
    pub fn new(code: HttpStatusCode) -> Self {
        Self { code }
    }

    pub fn encode<W: io::Write>(&self, buf: &mut W) -> io::Result<()> {
        write!(
            buf,
            "HTTP/1.1 {} {}\r\n",
            self.code as u16,
            self.code.as_text()
        )?;
        write!(buf, "\r\n")?;
        Ok(())
    }

    pub fn to_string(&self) -> String {
        let mut buf = Vec::with_capacity(512);
        self.encode(&mut buf).expect("Fail to write string");
        String::from_utf8(buf).expect("Invalid UTF8 string")
    }
}
