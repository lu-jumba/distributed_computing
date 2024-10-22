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
}
