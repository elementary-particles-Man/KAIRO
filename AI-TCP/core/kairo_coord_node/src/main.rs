use kairo_coord_node::CoordinationNode;

fn main() {
    let mut node = CoordinationNode::new_from_env();
    node.log_event("node_started", None, None);
    println!("Coordination Node {} started", node.node_id);
    // Placeholder for future REST/gRPC API
}
