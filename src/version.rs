
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum RtspVersion {
    /// `RTSP/1.0`
    Rtsp10
}

impl fmt::Display for RtspVersion {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            RtspVersion::Rtsp10 => "RTSP/1.0"
        })
    }
}


impl Default for RtspVersion {
    fn default() -> RtspVersion {
        RtspVersion::Rtsp10
    }
}

