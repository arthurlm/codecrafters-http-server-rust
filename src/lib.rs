pub mod request;
pub mod response;

mod model {
    pub mod code;
    pub mod verb;
}

pub use model::{code::HttpStatusCode, verb::HttpVerb};
