use std::net::IpAddr;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum EndpointClass {
    Local,
    KnownTest,
    KnownPeer,
    Suspicious,
    Unknown,
}

#[allow(dead_code)]
pub fn classify_ip(ip: IpAddr) -> EndpointClass {
    if ip.is_loopback() {
        return EndpointClass::Local;
    }

    if matches!(ip, IpAddr::V4(v4) if v4.is_private()) {
        return EndpointClass::Local;
    }

    if matches!(ip, IpAddr::V4(v4) if v4.octets() == [93, 184, 216, 34]) {
        return EndpointClass::KnownTest;
    }

    EndpointClass::Unknown
}
