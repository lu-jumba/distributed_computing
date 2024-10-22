struct Node {
    node_id: String,
    user: User, 
    available_ram: u64, // Total available RAM
    allocated_ram: u64, // RAM allocated to the system
    available_storage: u64, // Total available storage
    allocated_storage: u64, // Storage allocated for the system
    is_active: bool,
}

impl NodeManager {
    // Activate a node
    fn activate_node(node: &mut Node) {
        node.is_active = true;
        // Initialize communication with other nodes
        CommunicationLayer::register_node(node);
    }

    // Deactivate a node
    fn deactivate_node(node: &mut Node) {
        node.is_active = false;
        // Unregister node from the distributed system
        CommunicationLayer::unregister_node(node);
    }

    // Adjust RAM/Storage allocation
    fn set_resource_limits(node: &mut Node, ram_limit: u64, storage_limit: u64) {
        node.allocated_ram = ram_limit;
        node.allocated_storage = storage_limit;
    }
    
    // Release 80% of RAM when idle (user-defined condition)
    fn release_idle_resources(node: &mut Node) {
        let idle_ram_release = node.allocated_ram * 0.8;
        node.allocated_ram -= idle_ram_release;
    }
}
