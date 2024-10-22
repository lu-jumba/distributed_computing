use flate2::{write::GzEncoder, Compression};
use std::io::prelude::*;

struct Task {
    task_id: String,
    pub deadline: u64,      // Deadline timestamp (UNIX time)
    required_ram: u64,     // RAM needed for the task (in MB)
    required_cpu: u64,     // CPU usage required (in %)
    required_bandwidth: u64, // Bandwidth required (in Mbps)
    data: Vec<u8>,         // Task data (payload)
    assigned_node_id: Option<String>,  // Node to which this task is assigned
    retries: u32,          // Number of retries attempted
    max_retries: u32,      // Maximum retries allowed
}

impl Task {
    // Constructor
    fn new(task_id: &str, ram: u64, cpu: u64, bandwidth: u64, data: Vec<u8>) -> Task {
        Task {
            task_id: task_id.to_string(),
            required_ram: ram,
            required_cpu: cpu,
            required_bandwidth: bandwidth,
            data,
            assigned_node_id: None,
            retries: 0,
            max_retries: 3,  // Default retry limit of 3
        }
    }
    pub fn new(task_id: &str, priority: u8, ram: u64, cpu: u64, bandwidth: u64, data: Vec<u8>) -> Self {
        Task {
            task_id: task_id.to_string(),
            priority,
            required_ram: ram,
            required_cpu: cpu,
            required_bandwidth: bandwidth,
            data,
        }
    }

    // Check if the task has exceeded its retry limit
    fn exceeded_retry_limit(&self) -> bool {
        self.retries >= self.max_retries
    }

   
        // Check if a task is CPU-bound
        pub fn is_cpu_bound(&self) -> bool {
            self.required_cpu > self.required_ram / 10 // Example heuristic
        }
       
        
        // Compress task data before sending to nodes
        fn compress_task_data(data: &[u8]) -> Vec<u8> {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data).unwrap();
            encoder.finish().unwrap()
        }

}
