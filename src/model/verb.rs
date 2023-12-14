use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HttpVerb {
    Head,
    Options,
    Get,
    Put,
    Post,
    Delete,
}

impl HttpVerb {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(HttpVerb::Head, tag("HEAD")),
            value(HttpVerb::Options, tag("OPTIONS")),
            value(HttpVerb::Get, tag("GET")),
            value(HttpVerb::Put, tag("PUT")),
            value(HttpVerb::Post, tag("POST")),
            value(HttpVerb::Delete, tag("DELETE")),
        ))(input)
    }

    pub fn as_http_text(&self) -> &'static str {
        match self {
            HttpVerb::Head => "HEAD",
            HttpVerb::Options => "OPTIONS",
            HttpVerb::Get => "GET",
            HttpVerb::Put => "PUT",
            HttpVerb::Post => "POST",
            HttpVerb::Delete => "DELETE",
        }
    }
}
