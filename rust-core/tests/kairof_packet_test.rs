use kairof::build_pcap;

#[test]
fn test_kairof_build_pcap() {
    let pcap = build_pcap();
    assert!(pcap.len() > 0);
}
