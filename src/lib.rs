mod error;
pub mod file_server;
pub mod request;
pub mod response;

mod model {
    pub mod code;
    pub mod headers;
    pub mod verb;
}

pub use error::HttpServerError;
pub use model::{code::HttpStatusCode, headers::HttpHeader, verb::HttpVerb};
