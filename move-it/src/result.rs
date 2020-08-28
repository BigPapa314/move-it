use std::{error, fmt, result};

// Just a generic Result type to ease error handling for us. Errors in multithreaded
// async contexts needs some extra restrictions
pub type Result<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub text: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub fn error(text: impl Into<String>) -> Box<Error> {
    Box::new(Error { text: text.into() })
}
