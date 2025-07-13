use kairo_rust_core::coordination::node_manager::NodeManager;

#[test]
fn test_register_node() {
    let manager = NodeManager::new();
    let pk1 = vec![1u8, 2, 3];
    let node1 = manager.register_node(pk1.clone()).expect("registration failed");
    assert_eq!(node1.public_key, pk1);
    assert_eq!(node1.virtual_ip, "100.64.0.1");

    let pk2 = vec![4u8, 5, 6];
    let node2 = manager.register_node(pk2.clone()).expect("registration failed");
    assert_eq!(node2.public_key, pk2);
    assert_eq!(node2.virtual_ip, "100.64.0.2");
    assert_ne!(node1.id, node2.id);
}
