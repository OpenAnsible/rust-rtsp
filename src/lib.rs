/// rtsp_URL  =   ( "rtsp:" | "rtspu:" ) "//" host [ ":" port ] [ abs_path ]

/// RTSP URL Scheme
/// tcp: rtsp://  | udp: rtspu://

/// RTSP Port
/// Default port 554

/// For example, the RTSP URL:
/// rtsp://media.example.com:554/twister/audiotrack
/// rtspu://media.example.com:554/twister/audiotrack



pub mod status;
pub mod method;
pub mod version;
pub mod header;

pub mod request;
pub mod response;

pub mod error;

pub mod server;
pub mod client;