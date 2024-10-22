use std::collections::HashMap;
use crate::node::Node;

struct NodeController {
    nodes: HashMap<String, Node>,  // Map node_id -> Node
    failed_nodes: HashMap<String, Node>,  // Track failed nodes
}

impl NodeController {
    // Register a node with the controller
    pub fn register_node(&mut self, node: Node) {
        self.nodes.insert(node.node_id.clone(), node);
        println!("Node {} registered.", node.node_id);
    }

    // Mark node as failed if it doesn’t send heartbeat
    pub fn mark_node_as_failed(&mut self, node_id: &str) {
        if let Some(failed_node) = self.nodes.remove(node_id) {
            self.failed_nodes.insert(node_id.to_string(), failed_node);
            println!("Node {} marked as failed.", node_id);
        }
    }

    // Check if a node is available for task assignment
    pub fn is_node_available(&self, node_id: &str) -> bool {
        self.nodes.contains_key(node_id)
    }

    // Get a list of available nodes for task assignment
    pub fn get_available_nodes(&self) -> Vec<&Node> {
        self.nodes.values().collect()
    }

    // Handle node failure and reassign tasks
    pub fn handle_node_failure(&mut self, node_id: &str, task_tracker: &mut TaskTracker, load_balancer: &mut LoadBalancer, task_queue: &mut TaskQueue) {
        // Get all tasks assigned to the failed node
        for (task_id, assigned_node) in task_tracker.task_node_map.clone() {
            if assigned_node == node_id {
                println!("Reassigning Task {} from failed Node {}", task_id, node_id);

                // Retrieve task and push it back to the task queue
                if let Some(task) = task_queue.get_task_by_id(&task_id) {
                    task_queue.enqueue(task);
                    task_tracker.remove_task_assignment(&task_id);

                    // Reassign task to a different node
                    let available_nodes = self.get_available_nodes();
                    if let Some(node_id) = load_balancer.assign_task_to_node(&task, &mut available_nodes) {
                        println!("Task {} reassigned to Node {}", task_id, node_id);
                    } else {
                        println!("No available nodes for reassignment of Task {}", task_id);
                    }
                }
            }
            // Mark the node as failed
        self.mark_node_as_failed(node_id);
    }
}

// Add a new node dynamically
pub fn add_node(&mut self, node: Node) {
    self.nodes.insert(node.node_id.clone(), node);
    println!("Node {} added to the network.", node.node_id);
}

// Remove a node from the network
pub fn remove_node(&mut self, node_id: &str) {
    if let Some(node) = self.nodes.remove(node_id) {
        println!("Node {} removed from the network.", node_id);
        // Reassign tasks from the removed node
        self.reassign_tasks_from_node(&node);
    } else {
        println!("Node {} not found in the network.", node_id);
    }
}

// Reassign tasks when a node is removed
fn reassign_tasks_from_node(&mut self, node: &Node) {
    let mut tasks_to_reassign = vec![];

    for (task_id, assigned_node) in task_tracker.task_node_map.iter() {
        if assigned_node == &node.node_id {
            tasks_to_reassign.push(task_id.clone());
        }
    }

    // Reassign the tasks
    for task_id in tasks_to_reassign {
        if let Some(task) = task_queue.get_task_by_id(&task_id) {
            let available_nodes = self.get_available_nodes();
            load_balancer.assign_task_to_node(task, available_nodes);
        }
    }
}
}
