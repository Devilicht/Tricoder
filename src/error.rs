use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error{
    #[error("Usage: tricoder <kerkour.com>")]
    CliUsage,
    #[error("Request: {0")]
    Request(String),
}

impl std::convert::From<request::Error> for Error{
    fn from (err: request::Error) -> Self{
        Error::Request(err.to_string())
    }
}