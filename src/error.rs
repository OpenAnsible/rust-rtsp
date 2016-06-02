
use std::fmt;
use std::error::Error as StdError;
use std::io::Error as IoError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;


pub type Result<T> = ::std::result::Result<T, Error>;

#[doc(hidden)]
pub enum Void {}

#[derive(Debug)]
pub enum Error {
    Method,
    Uri(String),
    Version,
    Header,
    TooLarge,
    Incomplete,
    Status,
    Timeout,
    Io(IoError),
    /// Parsing a field as string failed
    Utf8(Utf8Error),
    #[doc(hidden)]
    __Nonexhaustive(Void)
}


impl fmt::Debug for Void {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}


impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Method => "Invalid Method specified",
            Error::Version => "Invalid RTSP version specified",
            Error::Header => "Invalid Header provided",
            Error::TooLarge => "Message head is too large",
            Error::Status => "Invalid Status provided",
            Error::Incomplete => "Message is incomplete",
            Error::Timeout => "Timeout",
            // Error::Uri(ref e) => e.description(),
            Error::Uri(ref e) => e.as_ref(),
            Error::Io(ref e) => e.description(),
            Error::Utf8(ref e) => e.description(),
            Error::__Nonexhaustive(ref void) =>  match *void {}
        }
    }
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref error) => Some(error),
            // Error::Uri(ref error) => Some(error),
            _ => None,
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

// impl From<url::ParseError> for Error {
//     fn from(err: url::ParseError) -> Error {
//         Uri(err)
//     }
// }

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Utf8(err.utf8_error())
    }
}


