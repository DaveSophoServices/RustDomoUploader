use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    InvalidSomething,
    NotFound,
}

impl fmt::Display for Error {
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {
	match *self {
	    Error::InvalidSomething => f.write_str("InvalidSomething"),
	    Error::NotFound => f.write_str("NotFound"),
	}
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
	match *self {
	    Error::InvalidSomething => "Something is Invalid",
	    Error::NotFound => "Not Found",
	}
    }
}
