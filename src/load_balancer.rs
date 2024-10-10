use futures::future::join_all;

impl LoadBalancer {

    // Check node capacity before assigning task
    pub fn assign_task_to_node(&mut self, task: &mut Task, available_nodes: &mut Vec<Node>) -> Option<String> {
        for node in available_nodes.iter_mut() {
            if node.can_handle_task(task) {
                // Assign task to node
                node.allocated_ram += task.required_ram;
                node.allocated_cpu += task.required_cpu;
                node.allocated_bandwidth += task.required_bandwidth;
                return Some(node.node_id.clone());
            }
        }

        None
    }

    // Differentiate between CPU-bound and memory-bound tasks for better load distribution
    pub fn assign_task_to_best_node(&mut self, task: &Task, available_nodes: &mut Vec<Node>) -> Option<String> {
        if task.is_cpu_bound() {
            // Prioritize nodes with more CPU availability
            available_nodes.sort_by_key(|node| node.available_cpu - node.allocated_cpu);
        } else {
            // Prioritize nodes with more RAM availability
            available_nodes.sort_by_key(|node| node.available_ram - node.allocated_ram);
        }

        // Assign task to the best-fit node
        for node in available_nodes.iter_mut() {
            if node.can_handle_task(task) {
                node.allocate_resources(task);
                return Some(node.node_id.clone());
            }
        }

        None
    }



    // Parallel task assignment to multiple nodes
    pub async fn assign_tasks_in_parallel(
        &mut self,
        tasks: Vec<Task>,
        available_nodes: &mut Vec<Node>,
    ) {
        let futures: Vec<_> = tasks
            .into_iter()
            .map(|mut task| async {
                let node_id = self.assign_task_to_best_node(&mut task, available_nodes);
                if let Some(node_id) = node_id {
                    println!("Task {} assigned to Node {}", task.task_id, node_id);
                } else {
                    println!("No suitable node found for Task {}", task.task_id);
                }
            })
            .collect();

        // Wait for all tasks to be assigned in parallel
        join_all(futures).await;
    }
}

    impl Task {
        // Check if a task is CPU-bound
        pub fn is_cpu_bound(&self) -> bool {
            self.required_cpu > self.required_ram / 10 // Example heuristic
        }
}

