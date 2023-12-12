use nom::{bytes::complete::take, bytes::complete::take_until, IResult};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

impl HttpHeader {
    pub fn new<K: AsRef<str>, V: AsRef<str>>(name: K, value: V) -> Self {
        Self {
            name: name.as_ref().to_ascii_lowercase(),
            value: value.as_ref().to_string(),
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, raw_name) = take_until(":")(input)?;
        let (input, _) = take(1_usize)(input)?;
        let (input, value) = take_until("\r\n")(input)?;
        let (input, _) = take(2_usize)(input)?;

        Ok((
            input,
            Self {
                name: raw_name.to_ascii_lowercase(),
                value: value.trim().to_string(),
            },
        ))
    }
}
