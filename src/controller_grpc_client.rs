use tonic::transport::Channel;
use node::node_service_client::NodeServiceClient;
use node::{HeartbeatRequest, TaskRequest};

pub mod node {
    tonic::include_proto!("node");
}

pub struct NodeController {
    client: NodeServiceClient<Channel>,
}

impl NodeController {
    pub async fn new(addr: String) -> Self {
        let client = NodeServiceClient::connect(addr).await.unwrap();
        Self { client }
    }

    // Check node health
    pub async fn check_node_health(&mut self, node_id: String) -> bool {
        let request = tonic::Request::new(HeartbeatRequest { node_id });
        let response = self.client.heartbeat(request).await.unwrap();
        response.into_inner().healthy
    }

    // Assign task to node
    pub async fn assign_task_to_node(
        &mut self,
        task_id: String,
        ram: u64,
        cpu: u64,
        bandwidth: u64,
        data: Vec<u8>,
    ) -> bool {
        let request = tonic::Request::new(TaskRequest {
            task_id,
            required_ram: ram,
            required_cpu: cpu,
            required_bandwidth: bandwidth,
            data,
        });
        let response = self.client.assign_task(request).await.unwrap();
        response.into_inner().success
    }

    async fn distribute_tasks(
        redis_client: &RedisClient,
        controller: &mut NodeController,
        task_id: String,
        task_ram: u64,
        task_cpu: u64,
        task_bandwidth: u64,
        task_data: Vec<u8>,
    ) {
        // Retrieve available nodes from Redis
        let node_ids = vec!["node_1".to_string(), "node_2".to_string()];
        
        for node_id in node_ids {
            if let Some((available_ram, available_cpu, available_bandwidth)) = redis_client.get_node_status(node_id.clone()).await {
                if available_ram >= task_ram && available_cpu >= task_cpu && available_bandwidth >= task_bandwidth {
                    // Assign the task via gRPC
                    let success = controller.assign_task_to_node(
                        task_id.clone(),
                        task_ram,
                        task_cpu,
                        task_bandwidth,
                        task_data.clone(),
                    ).await;
    
                    if success {
                        // Store task assignment in Redis
                        redis_client.assign_task_to_node(task_id.clone(), node_id.clone()).await;
                        println!("Task {} assigned to Node {}", task_id, node_id);
                        break;
                    }
                }
            }
        }
    }
    
}
