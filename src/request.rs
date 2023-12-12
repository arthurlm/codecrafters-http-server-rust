use nom::{bytes::complete::take_until, character::complete::space1, IResult};

use crate::HttpVerb;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpRequest {
    pub verb: HttpVerb,
    pub target: String,
}

impl HttpRequest {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, verb) = HttpVerb::parse(input)?;
        let (input, _) = space1(input)?;
        let (input, target) = take_until(" HTTP/1.1\r\n")(input)?;

        Ok((
            input,
            Self {
                verb,
                target: target.to_string(),
            },
        ))
    }
}
