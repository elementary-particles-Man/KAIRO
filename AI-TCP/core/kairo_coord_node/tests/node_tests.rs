use kairo_coord_node::CoordinationNode;
use uuid::Uuid;

#[test]
fn test_uuid_generation() {
    std::env::remove_var("KAIRO_NODE_ID");
    let node = CoordinationNode::new_from_env();
    assert!(Uuid::parse_str(&node.node_id.to_string()).is_ok());
}

#[test]
fn test_peer_add_remove() {
    let mut node = CoordinationNode::new_from_env();
    let peer = node.add_peer("testkey".to_string());
    assert_eq!(node.get_peers().len(), 1);
    assert!(node.remove_peer(&peer.uuid));
    assert_eq!(node.get_peers().len(), 0);
}
