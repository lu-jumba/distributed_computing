use futures::future::join_all;
use crate::task_queue::Task;
use crate::node::Node;

struct LoadBalancer;

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

    pub async fn assign_task_with_retry(
        &mut self,
        task: &mut Task,
        available_nodes: &mut Vec<Node>,
        task_queue: &mut TaskQueue,
        task_tracker: &mut TaskTracker,
        controller: &mut NodeController,
    ) {
        while !task.exceeded_retry_limit() {
            if let Some(node_id) = self.assign_task_to_node(task, available_nodes) {
                task_tracker.assign_task_to_node(&task.task_id, &node_id);
                info!("Task {} assigned to Node {}", task.task_id, node_id);

                let task_result = if node_id == "node_1" {
                    node1.execute_task(task).await
                } else {
                    node2.execute_task(task).await
                };

                match task_result {
                    Ok(_) => {
                        info!("Task {} completed successfully on retry #{}", task.task_id, task.retries);
                        task_tracker.remove_task_assignment(&task.task_id);
                        break;
                    }
                    Err(_) => {
                        task.retries += 1;
                        warn!("Retrying Task {} (Attempt #{}/{})", task.task_id, task.retries, task.max_retries);
                    }
                }
            } else {
                warn!("No available nodes for Task {}", task.task_id);
                break;
            }
        }

        if task.exceeded_retry_limit() {
            error!("Task {} permanently failed after {} retries", task.task_id, task.max_retries);
        }
    }

    // Enhanced logic for assigning tasks to the best available node
    pub fn assign_task_to_best_node(&mut self, task: &Task, available_nodes: &mut Vec<Node>) -> Option<String> {
        // Sort nodes based on available resources (e.g., CPU, RAM)
        available_nodes.sort_by_key(|node| node.available_ram - node.allocated_ram);
        
        for node in available_nodes.iter_mut() {
            // Check if node has enough resources to handle the task
            if node.allocated_ram + task.required_ram <= node.available_ram &&
               node.allocated_cpu + task.required_cpu <= node.available_cpu &&
               node.allocated_bandwidth + task.required_bandwidth <= node.available_bandwidth {
                // Assign the task to this node
                node.allocated_ram += task.required_ram;
                node.allocated_cpu += task.required_cpu;
                node.allocated_bandwidth += task.required_bandwidth;
                return Some(node.node_id.clone());
            }
        }

        // No suitable node found
        None
    }

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
    // Prioritize higher-priority tasks
    pub fn assign_tasks_with_priority(
        &mut self,
        tasks: &mut Vec<Task>,
        available_nodes: &mut Vec<Node>,
    ) {
        // Sort tasks by priority (descending)
        tasks.sort_by(|a, b| b.priority.cmp(&a.priority));

        for task in tasks {
            if let Some(node_id) = self.assign_task_to_best_node(task, available_nodes) {
                println!("Task {} (priority {}) assigned to Node {}", task.task_id, task.priority, node_id);
            } else {
                println!("No suitable node found for Task {}", task.task_id);
            }
        }
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

    pub fn assign_tasks_based_on_weight(
        &mut self,
        tasks: &mut Vec<Task>,
        available_nodes: &mut Vec<Node>,
    ) {
        // Sort nodes by weight (descending)
        available_nodes.sort_by(|a, b| b.weight.cmp(&a.weight));

        // Assign tasks to higher-weighted nodes first
        for task in tasks {
            if let Some(node_id) = self.assign_task_to_best_node(task, available_nodes) {
                println!("Task {} assigned to Node {} (weight {})", task.task_id, node_id, available_nodes.iter().find(|n| n.node_id == node_id).unwrap().weight);
            } else {
                println!("No suitable node found for Task {}", task.task_id);
            }
        }
    }
    pub fn assign_tasks_by_deadline(
        &mut self,
        tasks: &mut Vec<Task>,
        available_nodes: &mut Vec<Node>,
    ) {
        // Sort tasks by deadline (ascending)
        tasks.sort_by(|a, b| a.deadline.cmp(&b.deadline));

        for task in tasks {
            if let Some(node_id) = self.assign_task_to_best_node(task, available_nodes) {
                println!("Task {} with deadline {} assigned to Node {}", task.task_id, task.deadline, node_id);
            } else {
                println!("No suitable node found for Task {}", task.task_id);
            }
        }
    }
    pub fn assign_tasks_to_least_loaded_node(
        &mut self,
        tasks: &mut Vec<Task>,
        available_nodes: &mut Vec<Node>,
    ) {
        // Sort nodes by load (ascending)
        available_nodes.sort_by(|a, b| a.calculate_load().partial_cmp(&b.calculate_load()).unwrap());

        for task in tasks {
            if let Some(node_id) = self.assign_task_to_best_node(task, available_nodes) {
                println!("Task {} assigned to least loaded Node {}", task.task_id, node_id);
            } else {
                println!("No suitable node found for Task {}", task.task_id);
            }
        }
    }

    impl Task {
        // Check if a task is CPU-bound
        pub fn is_cpu_bound(&self) -> bool {
            self.required_cpu > self.required_ram / 10 // Example heuristic
        }
    
    }
}




    
