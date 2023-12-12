use nom::{branch::alt, bytes::complete::tag, IResult};

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
        let (input, value) = alt((
            tag("HEAD"),
            tag("OPTIONS"),
            tag("GET"),
            tag("PUT"),
            tag("POST"),
            tag("DELETE"),
        ))(input)?;

        let output = match value {
            "HEAD" => HttpVerb::Head,
            "OPTIONS" => HttpVerb::Options,
            "GET" => HttpVerb::Get,
            "PUT" => HttpVerb::Put,
            "POST" => HttpVerb::Post,
            "DELETE" => HttpVerb::Delete,
            _ => unreachable!(),
        };

        Ok((input, output))
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
