use rust_core::ai_tcp_packet_generated::aitcp::AITcpPacket;
use rust_core::packet_validator::validate_packet;

#[test]
fn test_validate_packet_success() {
    let packet = AITcpPacket::default(); // Fill with valid test data
    assert!(validate_packet(&packet).is_ok());
}

#[test]
fn test_validate_packet_fail() {
    let packet = AITcpPacket::default(); // Fill with invalid test data
    assert!(validate_packet(&packet).is_err());
}
