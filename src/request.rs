// extern crate method;
// extern crate status;


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