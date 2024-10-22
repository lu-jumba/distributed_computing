use tokio::task;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;

pub struct Node {
    pub node_id: String,
    pub weight: u8, // Weight of the node (higher value = more capable)
    pub available_ram: u64,
    pub allocated_ram: u64,
    pub available_storage: u64,
    pub allocated_storage: u64,
    pub available_cpu: u64, // in percentage (0 to 100)
    pub allocated_cpu: u64, // CPU percentage allocated to the system
    pub available_bandwidth: u64, // Available network bandwidth (in Mbps)
    pub allocated_bandwidth: u64, // Allocated bandwidth to the system (in Mbps)

    
}


pub struct Node {
    task_cache: HashMap<String, Vec<u8>>, // Task ID -> Cached data
}

impl Node {
    // Constructor for initializing a new node
    pub fn new(node_id: &str, available_ram: u64, available_storage: u64, available_cpu: u64, available_bandwidth: u64) -> Node {
        Node {
            node_id: node_id.to_string(),
            available_ram,
            allocated_ram: 0,
            available_storage,
            allocated_storage: 0,
            available_cpu,
            allocated_cpu: 0,
            available_bandwidth,
            allocated_bandwidth: 0,
            weight: 0,
        }
    }

    // Set CPU and Bandwidth Limits
    pub fn set_cpu_bandwidth_limits(&mut self, cpu_limit: u64, bandwidth_limit: u64) {
        if cpu_limit <= self.available_cpu {
            self.allocated_cpu = cpu_limit;
        } else {
            println!("Error: CPU limit exceeds available CPU capacity!");
        }
        
        if bandwidth_limit <= self.available_bandwidth {
            self.allocated_bandwidth = bandwidth_limit;
        } else {
            println!("Error: Bandwidth limit exceeds available network capacity!");
        }
    }

    // Simulate sending heartbeat
    pub async fn send_heartbeat(&self) -> Result<(), String> {
        loop {
            // Attempt to send heartbeat and timeout if it takes too long (indicating node failure)
            match timeout(Duration::from_secs(6), async {
                println!(
                    "Node {} sending heartbeat. Resources: RAM: {}MB, CPU: {}%, Bandwidth: {}Mbps",
                    self.node_id, self.allocated_ram, self.allocated_cpu, self.allocated_bandwidth
                );
                // Simulating heartbeat success/failure
                Ok::<(), String>(()) // Simulating successful heartbeat
            })
            .await
            {
                Ok(_) => { /* Heartbeat successful */ }
                Err(_) => {
                    // Heartbeat timeout means node failed
                    return Err(format!("Node {} failed to send heartbeat!", self.node_id));
                }
            }

            // Sleep for 5 seconds before sending the next heartbeat
            sleep(Duration::from_secs(5)).await;
        }
}

    // Send heartbeat with logging
    pub async fn send_heartbeat(&self) -> Result<(), String> {
        loop {
            match timeout(Duration::from_secs(6), async {
                info!("Node {} sending heartbeat.", self.node_id);
                Ok::<(), String>(())
            }).await {
                Ok(_) => { /* Heartbeat successful */ }
                Err(_) => {
                    warn!("Node {} failed to send heartbeat!", self.node_id);
                    return Err(format!("Node {} failed to send heartbeat!", self.node_id));
                }
            }
            sleep(Duration::from_secs(5)).await;
        }
    }

    // Execute task with logging
    pub async fn execute_task(&mut self, task: &mut Task) -> Result<(), String> {
        info!(
            "Node {} executing Task {}: {}MB RAM, {}% CPU, {}Mbps Bandwidth",
            self.node_id, task.task_id, task.required_ram, task.required_cpu, task.required_bandwidth
        );
        
        // Simulate task execution
        sleep(Duration::from_secs(5)).await;
        let result = simulate_task_execution();

        match result {
            Ok(_) => {
                info!("Task {} completed on Node {}", task.task_id, self.node_id);
                self.free_resources(task);
                Ok(())
            }
            Err(err) => {
                warn!("Task {} failed on Node {}: {}", task.task_id, self.node_id, err);
                Err(err)
            }
        }
    }


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
// Cache task data on the node
pub fn cache_task_data(&mut self, task_id: &str, data: Vec<u8>) {
    self.task_cache.insert(task_id.to_string(), data);
}

// Retrieve cached task data if available
pub fn get_cached_task_data(&self, task_id: &str) -> Option<&Vec<u8>> {
    self.task_cache.get(task_id)
}

// Batch heartbeats and send them periodically
pub async fn send_batched_heartbeat(&self) {
    loop {
        // Send heartbeat every 30 seconds instead of constant updates
        println!("Node {} sending batched heartbeat", self.node_id);
        sleep(Duration::from_secs(30)).await;
    }
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

// Allocate resources based on fair share principle
pub fn allocate_fair_share_resources(&mut self, tasks: &mut Vec<Task>) {
    let total_tasks = tasks.len() as u64;

    // Allocate fair share of RAM and CPU to each task
    for task in tasks.iter_mut() {
        let fair_ram = self.available_ram / total_tasks;
        let fair_cpu = self.available_cpu / total_tasks;

        task.required_ram = fair_ram.min(task.required_ram);
        task.required_cpu = fair_cpu.min(task.required_cpu);

        println!("Task {} assigned fair share: {}MB RAM, {}% CPU", task.task_id, task.required_ram, task.required_cpu);
    }
}

// Calculate the current load on the node (as a percentage)
pub fn calculate_load(&self) -> f64 {
    let cpu_load = (self.allocated_cpu as f64 / self.available_cpu as f64) * 100.0;
    let ram_load = (self.allocated_ram as f64 / self.available_ram as f64) * 100.0;
    let bandwidth_load = (self.allocated_bandwidth as f64 / self.available_bandwidth as f64) * 100.0;
    (cpu_load + ram_load + bandwidth_load) / 3.0 // Average load across CPU, RAM, and bandwidth
}

// Placeholder function to pause a task
fn pause_task(&self, task: &Task) {
    println!("Task {} paused on Node {}", task.task_id, self.node_id);
}
}



