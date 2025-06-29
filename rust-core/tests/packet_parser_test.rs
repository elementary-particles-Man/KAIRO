use rust_core::packet_parser::PacketParser;

#[test]
fn detects_sequence_mismatch() {
    let mut parser = PacketParser::new(vec![]);
    let mut packet1 = 1u64.to_be_bytes().to_vec();
    packet1.extend_from_slice(b"hello");
    let result = parser.parse(&packet1);
    assert!(result.is_ok());

    // Reusing same sequence id should trigger error
    let result_err = parser.parse(&packet1);
    assert!(result_err.is_err());
}
