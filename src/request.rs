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

use crate::{HttpHeader, HttpVerb};

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
}
