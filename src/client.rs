

use std::net::{ TcpStream, UdpSocket };
use method::Method;
use response::Response;

// TCP
#[derive(Debug)]
pub struct Rtsp {
    uri    : String,
    session: Option<String>
}

// UDP
#[derive(Debug)]
pub struct Rtspu {
    uri    : String,
    session: Option<String>
}


impl Rtsp {
    pub fn new (uri: &str) -> Rtsp {
        Rtsp { uri: uri.to_string(), session: None }
    }
    pub fn get_uri(&self) -> String {
        self.uri.clone().to_string()
    }
    pub fn request(&self, method: Method ) -> Result<Response, &'static str> {
        if !(method.is_c_to_s()) {
            return Err("Method Not Allowed.")
        }
        Ok(Response::new())
    }
}