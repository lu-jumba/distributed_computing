use flate2::{write::GzEncoder, Compression};
use std::io::prelude::*;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use tokio::task;

pub struct Node {
    task_cache: HashMap<String, Vec<u8>>, // Task ID -> Cached data
}

impl Node {

      // Node maintains its own task queue
      pub async fn add_task_to_queue(&mut self, task: Task) {
        if self.can_handle_task(&task) {
            // Execute the task concurrently using async
            let node_id = self.node_id.clone();
            task::spawn(async move {
                node.execute_task(task).await;
            });
        } else {
            println!("Node {} does not have enough resources for Task {}", self.node_id, task.task_id);
        }
    }

    // Check if the node has sufficient resources for the task
    fn can_handle_task(&self, task: &Task) -> bool {
        self.allocated_ram + task.required_ram <= self.available_ram &&
        self.allocated_cpu + task.required_cpu <= self.available_cpu &&
        self.allocated_bandwidth + task.required_bandwidth <= self.available_bandwidth
    }

    // Dynamically adjust resource allocation
    pub fn adjust_resources(&mut self) {
        let cpu_load = self.calculate_cpu_load();
        if cpu_load < 50 {
            // Allocate more bandwidth if CPU load is low
            self.allocated_bandwidth += 10;
        } else if cpu_load > 80 {
            // Reduce bandwidth to prioritize CPU-bound tasks
            self.allocated_bandwidth = self.available_bandwidth / 2;
        }
    }

    // Simulate calculating CPU load (would use actual metrics in a real system)
    fn calculate_cpu_load(&self) -> u64 {
        (self.allocated_cpu as f64 / self.available_cpu as f64 * 100.0) as u64
    }

    pub fn preemptive_task_scheduling(&mut self, new_task: &Task) {
        // Check if current tasks can be preempted
        for current_task in self.running_tasks.iter_mut() {
            if new_task.priority > current_task.priority {
                println!(
                    "Preempting Task {} with Task {} on Node {}",
                    current_task.task_id, new_task.task_id, self.node_id
                );
                // Preempt the current task
                self.pause_task(current_task);
                self.execute_task(new_task.clone());
                break;
            }
        }
    }

    // Batch heartbeats and send them periodically
    pub async fn send_batched_heartbeat(&self) {
        loop {
            // Send heartbeat every 30 seconds instead of constant updates
            println!("Node {} sending batched heartbeat", self.node_id);
            sleep(Duration::from_secs(30)).await;
        }
    }

// Compress task data before sending to nodes
fn compress_task_data(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

   // Cache task data on the node
   pub fn cache_task_data(&mut self, task_id: &str, data: Vec<u8>) {
    self.task_cache.insert(task_id.to_string(), data);
}

// Retrieve cached task data if available
pub fn get_cached_task_data(&self, task_id: &str) -> Option<&Vec<u8>> {
    self.task_cache.get(task_id)
}

    // Placeholder function to pause a task
    fn pause_task(&self, task: &Task) {
        println!("Task {} paused on Node {}", task.task_id, self.node_id);
    }


}

impl NodeController {
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

