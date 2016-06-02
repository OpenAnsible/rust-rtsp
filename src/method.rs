
use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use error::Error;


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Method {
    Options,
    Describe,
    Setup,
    Play,
    Pause,
    Record,
    Announce,
    Teardown,
    GetParameter,
    SetParameter,
    Redirect,
    // Method extensions. An example would be `let m = Extension("FOO".to_string())`.
    Extension(String)
}
impl Method {
    pub fn is_s_to_c(&self) -> bool {
        // https://tools.ietf.org/html/rfc2326#section-10
        match *self {
            Method::Options
            | Method::Announce
            | Method::GetParameter
            | Method::SetParameter
            | Method::Redirect
            | Method::Extension(..) => true,
            _ => false
        }
    }
    pub fn is_c_to_s(&self) -> bool {
        // https://tools.ietf.org/html/rfc2326#section-10
        match *self {
            Method::Options
            | Method::Describe
            | Method::Announce
            | Method::GetParameter
            | Method::Pause
            | Method::Play
            | Method::Record
            | Method::Setup
            | Method::SetParameter
            | Method::Teardown
            | Method::Extension(..) => true,
            _ => false
        }
    }
}
impl AsRef<str> for Method {
    fn as_ref(&self) -> &str {
        match *self {
            Method::Options  => "OPTIONS",
            Method::Describe => "DESCRIBE",
            Method::Setup    => "SETUP",
            Method::Play     => "PLAY",
            Method::Pause    => "PAUSE",
            Method::Record   => "RECORD",
            Method::Announce => "ANNOUNCE",
            Method::Teardown => "TEARDOWN",
            Method::GetParameter     => "GET_PARAMETER",
            Method::SetParameter     => "SET_PARAMETER",
            Method::Redirect         => "REDIRECT",
            Method::Extension(ref s) => s.as_ref()
        }
    }
}

impl FromStr for Method {
    type Err = Error;
    fn from_str(s: &str) -> Result<Method, Error> {
        if s == "" {
            Err(Error::Method)
        } else {
            Ok(match s {
                "OPTIONS"  => Method::Options,
                "DESCRIBE" => Method::Describe,
                "SETUP"    => Method::Setup,
                "PLAY"     => Method::Play,
                "PAUSE"    => Method::Pause,
                "RECORD"   => Method::Record,
                "ANNOUNCE" => Method::Announce,
                "TEARDOWN" => Method::Teardown,
                "GET_PARAMETER" => Method::GetParameter,
                "SET_PARAMETER" => Method::SetParameter,
                "REDIRECT"      => Method::Redirect,
                _               => Method::Extension(s.to_owned())
            })
        }
    }
}


impl fmt::Display for Method {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Method::Options  => "OPTIONS",
            Method::Describe => "DESCRIBE",
            Method::Setup    => "SETUP",
            Method::Play     => "PLAY",
            Method::Pause    => "PAUSE",
            Method::Record   => "RECORD",
            Method::Announce => "ANNOUNCE",
            Method::Teardown => "TEARDOWN",
            Method::GetParameter     => "GET_PARAMETER",
            Method::SetParameter     => "SET_PARAMETER",
            Method::Redirect         => "REDIRECT",
            Method::Extension(ref s) => s.as_ref()
        })
    }
}

impl Default for Method {
    fn default() -> Method {
        Method::Options
    }
}