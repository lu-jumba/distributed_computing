use std::collections::HashMap;

struct TaskTracker {
    task_node_map: HashMap<String, String>,  // Map task_id -> node_id
}

impl TaskTracker {
    // Assign a task to a node
    pub fn assign_task_to_node(&mut self, task_id: &str, node_id: &str) {
        self.task_node_map.insert(task_id.to_string(), node_id.to_string());
    }

    // Get the node currently handling a task
    pub fn get_assigned_node(&self, task_id: &str) -> Option<String> {
        self.task_node_map.get(task_id).cloned()
    }

    // Remove task assignment when task is completed or node fails
    pub fn remove_task_assignment(&mut self, task_id: &str) {
        self.task_node_map.remove(task_id);
    }
}
