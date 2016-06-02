use std::fmt;
use std::cmp::Ordering;


use ::method::Method;


// https://tools.ietf.org/html/rfc2326#section-7.1.1

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum StatusClass {
    /// 1xx (Informational): The request was received, continuing process
    Informational,
    /// 2xx (Success): The request was successfully received, understood, and accepted
    Success,
    /// 3xx (Redirection): Further action needs to be taken in order to complete the request
    Redirection,
    /// 4xx (Client Error): The request contains bad syntax or cannot be fulfilled
    ClientError,
    /// 5xx (Server Error): The server failed to fulfill an apparently valid request
    ServerError,
    /// A status code lower than 100 or higher than 599. These codes do no belong to any class.
    NoClass,
}

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
#[derive(Debug, Hash)]
pub enum StatusCode {
    Continue,         // 100
    Ok,               // 200
    Created,          // 201
    LowOnStorageSpace,// 250
    MultipleChoices,  // 300
    MovedPermanently, // 301
    MovedTemporarily, // 302
    SeeOther,         // 303
    NotModified,      // 304
    UseProxy,         // 305
    BadRequest,       // 400
    Unauthorized,     // 401
    PaymentRequired,  // 402
    Forbidden,        // 403
    NotFound,         // 404
    MethodNotAllowed, // 405
    NotAcceptable,    // 406
    ProxyAuthenticationRequired, // 407
    RequestTimeout,   // 408
    Gone,             // 410
    LengthRequired,   // 411
    PreconditionFailed,    // 412
    RequestEntityTooLarge, // 413
    RequestURITooLarge,    // 414
    UnsupportedMediaType,  // 415
    ParameterNotUnderstood,// 451
    ConferenceNotFound,    // 452
    NotEnoughBandwidth,    // 453
    SessionNotFound,       // 454
    MethodNotValidInThisState,      // 455
    HeaderFieldNotValidForResource, // 456
    InvalidRange,                   // 457
    ParameterIsReadOnly,            // 458
    AggregateOperationNotAllowed,   // 459
    OnlyAggregateOperationAllowed,  // 460
    UnsupportedTransport,           // 461
    DestinationUnreachable,         // 462
    InternalServerError,      // 500
    NotImplemented,           // 501
    BadGateway,               // 502
    ServiceUnavailable,       // 503
    GatewayTimeout,           // 504
    RTSPVersionNotSupported,  // 505
    OptionNotSupported,       // 551
    // Extension Code, 3DIGIT; Reason-Phrase  =     *<TEXT, excluding CR, LF>
    Extension(u16)
}

impl StatusCode {
    pub fn from_u16(n: u16) -> StatusCode {
        match n {
            100 => StatusCode::Continue,
            200 => StatusCode::Ok,
            201 => StatusCode::Created,
            250 => StatusCode::LowOnStorageSpace,
            300 => StatusCode::MultipleChoices,
            301 => StatusCode::MovedPermanently,
            302 => StatusCode::MovedTemporarily,
            303 => StatusCode::SeeOther,
            304 => StatusCode::NotModified,
            305 => StatusCode::UseProxy,
            400 => StatusCode::BadRequest,
            401 => StatusCode::Unauthorized,
            402 => StatusCode::PaymentRequired,
            403 => StatusCode::Forbidden,
            404 => StatusCode::NotFound,
            405 => StatusCode::MethodNotAllowed,
            406 => StatusCode::NotAcceptable,
            407 => StatusCode::ProxyAuthenticationRequired,
            408 => StatusCode::RequestTimeout,
            410 => StatusCode::Gone,
            411 => StatusCode::LengthRequired,
            412 => StatusCode::PreconditionFailed,
            413 => StatusCode::RequestEntityTooLarge,
            414 => StatusCode::RequestURITooLarge,
            415 => StatusCode::UnsupportedMediaType,
            451 => StatusCode::ParameterNotUnderstood,
            452 => StatusCode::ConferenceNotFound,
            453 => StatusCode::NotEnoughBandwidth,
            454 => StatusCode::SessionNotFound,
            455 => StatusCode::MethodNotValidInThisState,
            456 => StatusCode::HeaderFieldNotValidForResource,
            457 => StatusCode::InvalidRange,
            458 => StatusCode::ParameterIsReadOnly,
            459 => StatusCode::AggregateOperationNotAllowed,
            460 => StatusCode::OnlyAggregateOperationAllowed,
            461 => StatusCode::UnsupportedTransport,
            462 => StatusCode::DestinationUnreachable,
            500 => StatusCode::InternalServerError,
            501 => StatusCode::NotImplemented,
            502 => StatusCode::BadGateway,
            503 => StatusCode::ServiceUnavailable,
            504 => StatusCode::GatewayTimeout,
            505 => StatusCode::RTSPVersionNotSupported,
            551 => StatusCode::OptionNotSupported,
            _   => StatusCode::Extension(n)
        }
    }
    pub fn to_u16(&self) -> u16 {
        match *self {
            StatusCode::Continue => 100,
            StatusCode::Ok       => 200,
            StatusCode::Created  => 201,
            StatusCode::LowOnStorageSpace => 250,
            StatusCode::MultipleChoices   => 300,
            StatusCode::MovedPermanently  => 301,
            StatusCode::MovedTemporarily  => 302,
            StatusCode::SeeOther     => 303,
            StatusCode::NotModified  => 304,
            StatusCode::UseProxy     => 305,
            StatusCode::BadRequest   => 400,
            StatusCode::Unauthorized => 401,
            StatusCode::PaymentRequired  => 402,
            StatusCode::Forbidden        => 403,
            StatusCode::NotFound         => 404,
            StatusCode::MethodNotAllowed => 405,
            StatusCode::NotAcceptable    => 406,
            StatusCode::ProxyAuthenticationRequired => 407,
            StatusCode::RequestTimeout => 408,
            StatusCode::Gone => 410,
            StatusCode::LengthRequired        => 411,
            StatusCode::PreconditionFailed    => 412,
            StatusCode::RequestEntityTooLarge => 413,
            StatusCode::RequestURITooLarge    => 414,
            StatusCode::UnsupportedMediaType  => 415,
            StatusCode::ParameterNotUnderstood => 451,
            StatusCode::ConferenceNotFound => 452,
            StatusCode::NotEnoughBandwidth => 453,
            StatusCode::SessionNotFound    => 454,
            StatusCode::MethodNotValidInThisState      => 455,
            StatusCode::HeaderFieldNotValidForResource => 456,
            StatusCode::InvalidRange        => 457,
            StatusCode::ParameterIsReadOnly => 458,
            StatusCode::AggregateOperationNotAllowed  => 459,
            StatusCode::OnlyAggregateOperationAllowed => 460,
            StatusCode::UnsupportedTransport   => 461,
            StatusCode::DestinationUnreachable => 462,
            StatusCode::InternalServerError    => 500,
            StatusCode::NotImplemented => 501,
            StatusCode::BadGateway     => 502,
            StatusCode::ServiceUnavailable => 503,
            StatusCode::GatewayTimeout     => 504,
            StatusCode::RTSPVersionNotSupported => 505,
            StatusCode::OptionNotSupported      => 551,
            StatusCode::Extension(n) => n
        }
    }
    pub fn canonical_reason(&self) -> Option<&'static str> {
        match *self {
            StatusCode::Continue => Some("Continue"),
            StatusCode::Ok       => Some("OK"),
            StatusCode::Created  => Some("Created"),
            StatusCode::LowOnStorageSpace => Some("Low on Storage Space"),
            StatusCode::MultipleChoices   => Some("Multiple Choices"),
            StatusCode::MovedPermanently  => Some("Moved Permanently"),
            StatusCode::MovedTemporarily  => Some("Moved Temporarily"),
            StatusCode::SeeOther     => Some("See Other"),
            StatusCode::NotModified  => Some("Not Modified"),
            StatusCode::UseProxy     => Some("Use Proxy"),
            StatusCode::BadRequest   => Some("Bad Request"),
            StatusCode::Unauthorized => Some("Unauthorized"),
            StatusCode::PaymentRequired  => Some("Payment Required"),
            StatusCode::Forbidden        => Some("Forbidden"),
            StatusCode::NotFound         => Some("Not Found"),
            StatusCode::MethodNotAllowed => Some("Method Not Allowed"),
            StatusCode::NotAcceptable    => Some("Not Acceptable"),
            StatusCode::ProxyAuthenticationRequired => Some("Proxy Authentication Required"),
            StatusCode::RequestTimeout   => Some("Request Time-out"),
            StatusCode::Gone             => Some("Gone"),
            StatusCode::LengthRequired         => Some("Length Required"),
            StatusCode::PreconditionFailed     => Some("Precondition Failed"),
            StatusCode::RequestEntityTooLarge  => Some("Request Entity Too Large"),
            StatusCode::RequestURITooLarge     => Some("Request-URI Too Large"),
            StatusCode::UnsupportedMediaType   => Some("Unsupported Media Type"),
            StatusCode::ParameterNotUnderstood => Some("Parameter Not Understood"),
            StatusCode::ConferenceNotFound     => Some("Conference Not Found"),
            StatusCode::NotEnoughBandwidth     => Some("Not Enough Bandwidth"),
            StatusCode::SessionNotFound        => Some("Session Not Found"),
            StatusCode::MethodNotValidInThisState      => Some("Method Not Valid in This State"),
            StatusCode::HeaderFieldNotValidForResource => Some("Header Field Not Valid for Resource"),
            StatusCode::InvalidRange                   => Some("Invalid Range"),
            StatusCode::ParameterIsReadOnly            => Some("Parameter Is Read-Only"),
            StatusCode::AggregateOperationNotAllowed   => Some("Aggregate operation not allowed"),
            StatusCode::OnlyAggregateOperationAllowed  => Some("Only aggregate operation allowed"),
            StatusCode::UnsupportedTransport    => Some("Unsupported transport"),
            StatusCode::DestinationUnreachable  => Some("Destination unreachable"),
            StatusCode::InternalServerError     => Some("Internal Server Error"),
            StatusCode::NotImplemented          => Some("Not Implemented"),
            StatusCode::BadGateway              => Some("Bad Gateway"),
            StatusCode::ServiceUnavailable      => Some("Service Unavailable"),
            StatusCode::GatewayTimeout          => Some("Gateway Time-out"),
            StatusCode::RTSPVersionNotSupported => Some("RTSP Version not supported"),
            StatusCode::OptionNotSupported      => Some("Option not supported"),
            StatusCode::Extension(..) => None
        }
    }
    pub fn class(&self) -> StatusClass {
        match self.to_u16() {
            100...199 => StatusClass::Informational,
            200...299 => StatusClass::Success,
            300...399 => StatusClass::Redirection,
            400...499 => StatusClass::ClientError,
            500...599 => StatusClass::ServerError,
            _         => StatusClass::NoClass,
        }
    }

    pub fn is_safe_for_method(&self, method: Method) -> bool {
        match *self {
            StatusCode::Continue => true,  // all methods
            StatusCode::Ok       => true,
            StatusCode::Created  => {
                match method {
                    Method::Record => true,
                    _              => false
                }
            },
            StatusCode::LowOnStorageSpace => {
                match method {
                    Method::Record => true,
                    _              => false
                }
            },
            StatusCode::MultipleChoices   => true,
            StatusCode::MovedPermanently  => true,
            StatusCode::MovedTemporarily  => true,
            StatusCode::SeeOther     => true,
            StatusCode::NotModified  => true,
            StatusCode::UseProxy     => true,
            StatusCode::BadRequest   => true,
            StatusCode::Unauthorized => true,
            StatusCode::PaymentRequired  => true,
            StatusCode::Forbidden        => true,
            StatusCode::NotFound         => true,
            StatusCode::MethodNotAllowed => true,
            StatusCode::NotAcceptable    => true,
            StatusCode::ProxyAuthenticationRequired => true,
            StatusCode::RequestTimeout   => true,
            StatusCode::Gone             => true,
            StatusCode::LengthRequired         => true,
            StatusCode::PreconditionFailed     => {
                match method {
                    Method::Describe | Method::Setup => true,
                    _                => false
                }
            },
            StatusCode::RequestEntityTooLarge  => true,
            StatusCode::RequestURITooLarge     => true,
            StatusCode::UnsupportedMediaType   => true,
            StatusCode::ParameterNotUnderstood => match method {
                Method::Setup => true,
                _             => false
            },
            StatusCode::ConferenceNotFound     => match method {
                Method::Setup => true,
                _             => false
            },
            StatusCode::NotEnoughBandwidth     => match method {
                Method::Setup => true,
                _             => false
            },
            StatusCode::SessionNotFound        => true,
            StatusCode::MethodNotValidInThisState      => true,
            StatusCode::HeaderFieldNotValidForResource => true,
            StatusCode::InvalidRange                   => match method {
                Method::Play => true,
                _             => false
            },
            StatusCode::ParameterIsReadOnly            => match method {
                Method::SetParameter => true,
                _             => false
            },
            StatusCode::AggregateOperationNotAllowed   => true,
            StatusCode::OnlyAggregateOperationAllowed  => true,
            StatusCode::UnsupportedTransport    => true,
            StatusCode::DestinationUnreachable  => true,
            StatusCode::InternalServerError     => true,
            StatusCode::NotImplemented          => true,
            StatusCode::BadGateway              => true,
            StatusCode::ServiceUnavailable      => true,
            StatusCode::GatewayTimeout          => true,
            StatusCode::RTSPVersionNotSupported => true,
            StatusCode::OptionNotSupported      => true,
            StatusCode::Extension(..)           => true
        }
    }
    pub fn is_informational(&self) -> bool {
        self.class() == StatusClass::Informational
    }
    pub fn is_success(&self) -> bool {
        self.class() == StatusClass::Success
    }
    pub fn is_redirection(&self) -> bool {
        self.class() == StatusClass::Redirection
    }
    pub fn is_client_error(&self) -> bool {
        self.class() == StatusClass::ClientError
    }
    pub fn is_server_error(&self) -> bool {
        self.class() == StatusClass::ServerError
    }
    pub fn is_strange_status(&self) -> bool {
        self.class() == StatusClass::NoClass
    }
}

impl Copy for StatusCode {}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.to_u16(),
               self.canonical_reason().unwrap_or("<unknown status code>"))
    }
}

impl PartialEq for StatusCode {
    #[inline]
    fn eq(&self, other: &StatusCode) -> bool {
        self.to_u16() == other.to_u16()
    }
}


impl Eq for StatusCode {}

impl Clone for StatusCode {
    #[inline]
    fn clone(&self) -> StatusCode {
        *self
    }
}


impl PartialOrd for StatusCode {
    #[inline]
    fn partial_cmp(&self, other: &StatusCode) -> Option<Ordering> {
        self.to_u16().partial_cmp(&(other.to_u16()))
    }
}

impl Ord for StatusCode {
    #[inline]
    fn cmp(&self, other: &StatusCode) -> Ordering {
        if *self < *other {
            Ordering::Less
        } else if *self > *other {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Default for StatusCode {
    fn default() -> StatusCode {
        StatusCode::Ok
    }
}
