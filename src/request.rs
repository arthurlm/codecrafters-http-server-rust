use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{crlf, space1},
    combinator::eof,
    multi::many_till,
    number::complete::float,
    sequence::tuple,
    IResult,
};
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::{HttpHeader, HttpServerError, HttpVerb};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpRequest {
    pub verb: HttpVerb,
    pub target: String,
    pub headers: Vec<HttpHeader>,
}

impl HttpRequest {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, verb) = HttpVerb::parse(input)?;
        let (input, _) = space1(input)?;
        let (input, target) = take_until(" HTTP/")(input)?;
        let (input, (_, _, version, _)) = tuple((space1, tag("HTTP/"), float, crlf))(input)?;
        assert_eq!(version, 1.1, "Unsupported HTTP version");

        let (input, (headers, _)) = many_till(HttpHeader::parse, alt((crlf, eof)))(input)?;

        Ok((
            input,
            Self {
                verb,
                target: target.to_string(),
                headers,
            },
        ))
    }

    pub fn get_header(&self, name: &str) -> Option<&str> {
        let lower_name = name.to_ascii_lowercase();
        self.headers
            .iter()
            .find(|h| h.name == lower_name)
            .map(|h| h.value.as_str())
    }

    pub async fn read_content<R>(&self, reader: &mut R) -> Result<Vec<u8>, HttpServerError>
    where
        R: AsyncRead + Unpin,
    {
        let content_length: usize = self
            .get_header("content-length")
            .ok_or(HttpServerError::MissingHeader("content-length"))?
            .parse()?;

        let mut output = vec![0; content_length];
        reader.read_exact(&mut output).await?;
        Ok(output)
    }
}
