use crate::node::Node;

pub struct ResourceManager;

impl ResourceManager {
    // Monitor all resources, including CPU and Bandwidth
    pub fn monitor_resources(node: &Node) {
        println!("Node ID: {}", node.node_id);
        println!("Allocated RAM: {}MB out of {}MB", node.allocated_ram, node.available_ram);
        println!("Allocated Storage: {}GB out of {}GB", node.allocated_storage, node.available_storage);
        println!("Allocated CPU: {}% out of {}%", node.allocated_cpu, node.available_cpu);
        println!("Allocated Bandwidth: {}Mbps out of {}Mbps", node.allocated_bandwidth, node.available_bandwidth);
    }

    // Release a percentage of CPU usage when node is idle
    pub fn release_cpu_resources(node: &mut Node, retain_percentage: f64) {
        let retained_cpu = (node.allocated_cpu as f64 * retain_percentage).round() as u64;
        println!("Releasing CPU, retaining {}% out of {}%", retained_cpu, node.allocated_cpu);
        node.allocated_cpu = retained_cpu;
    }

    // Release bandwidth when idle
    pub fn release_bandwidth(node: &mut Node, retain_percentage: f64) {
        let retained_bandwidth = (node.allocated_bandwidth as f64 * retain_percentage).round() as u64;
        println!("Releasing Bandwidth, retaining {}Mbps out of {}Mbps", retained_bandwidth, node.allocated_bandwidth);
        node.allocated_bandwidth = retained_bandwidth;
    }
}

