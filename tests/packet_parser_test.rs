//! tests/packet_parser_test.rs

#[cfg(test)]
mod tests {
    use kairo_core::packet_parser::PacketParser; // Updated path

    #[test]
    fn test_parser_instantiation() {
        let _parser = PacketParser::new();
        // The test passes if new() does not panic
    }
}
