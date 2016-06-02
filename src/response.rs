
use method::Method;
use status::StatusCode;
use version::RtspVersion;
use header::Headers;


#[derive(Debug)]
pub struct Response {
    method : Method,
    status : StatusCode,
    version: RtspVersion,
    headers: Headers
}

impl Response {
    pub fn new () -> Response {
        Response {
            method: Method::Options,
            status: StatusCode::Ok,
            version: RtspVersion::Rtsp10,
            headers: Headers::new()
        }
    }
}