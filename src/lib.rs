pub struct RtpSession {
}

pub struct RtpEndPoint {
}

pub enum Payload {
    TestPayload
}

pub struct RtpHeader {
    csrc: u8,
    marker: bool,
    payload_type: u8,
    seq: u16,
    timestamp: u32,
    ssrc: u32,
}
