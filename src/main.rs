mod node;
mod resource_manager;
mod ipfs_storage;
mod communication_layer;
mod task_queue;
mod load_balancer;

use node::Node;
use resource_manager::ResourceManager;
use task_queue::TaskQueue;
use load_balancer::LoadBalancer;
use communication_layer::CommunicationLayer;
use tokio::join;

#[tokio::main]
async fn main() {
    // Initialize nodes and controller
    let mut node1 = Node::new("node_1", 8192, 256_000, 100, 50);
    let mut node2 = Node::new("node_2", 4096, 128_000, 80, 30);
    let mut node3 = Node::new("node_3", 10240, 512_000, 120, 100);

    let mut controller = NodeController { nodes: HashMap::new(), failed_nodes: HashMap::new() };
    let mut task_tracker = TaskTracker { task_node_map: HashMap::new() };

    // Add nodes to the system
    controller.add_node(node1.clone());
    controller.add_node(node2.clone());
    controller.add_node(node3.clone());

    // Task queue, task tracker, and load balancer
    let mut task_queue = TaskQueue { queue: VecDeque::new() };
    let mut load_balancer = LoadBalancer;

    // Add tasks to the queue
    let mut task1 = Task::new("task_1", 2000, 20, 10, vec![1, 2, 3, 4]);
    let mut task2 = Task::new("task_2", 1000, 10, 5, vec![5, 6, 7, 8]);
    let mut task3 = Task::new("task_3", 1500, 15, 10, vec![9, 10, 11, 12]);

    task_queue.enqueue(task1);
    task_queue.enqueue(task2);
    task_queue.enqueue(task3);

    // Process tasks and assign them in parallel across nodes
    load_balancer.assign_tasks_in_parallel(vec![task1, task2, task3], &mut controller.get_available_nodes()).await;

    // Wait for heartbeats (run indefinitely)
    let _ = join!(node1.send_batched_heartbeat(), node2.send_batched_heartbeat());
}
