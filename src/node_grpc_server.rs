use tonic::{transport::Server, Request, Response, Status};
use node::node_service_server::{NodeService, NodeServiceServer};
use node::{HeartbeatRequest, HeartbeatResponse, TaskRequest, TaskResponse, NodeStatusRequest, NodeStatusResponse};

pub mod node {
    tonic::include_proto!("node");
}

#[derive(Default)]
pub struct MyNodeService;

#[tonic::async_trait]
impl NodeService for MyNodeService {
    async fn heartbeat(
        &self,
        request: Request<HeartbeatRequest>,
    ) -> Result<Response<HeartbeatResponse>, Status> {
        let node_id = request.into_inner().node_id;
        println!("Heartbeat received from Node: {}", node_id);

        Ok(Response::new(HeartbeatResponse { healthy: true }))
    }

    async fn assign_task(
        &self,
        request: Request<TaskRequest>,
    ) -> Result<Response<TaskResponse>, Status> {
        let task_id = request.into_inner().task_id;
        println!("Task {} assigned", task_id);

        // Simulate task processing (In reality, you'd assign this task to the node's queue)
        Ok(Response::new(TaskResponse {
            success: true,
            message: "Task assigned successfully".to_string(),
        }))
    }

    async fn get_node_status(
        &self,
        _: Request<NodeStatusRequest>,
    ) -> Result<Response<NodeStatusResponse>, Status> {
        // Respond with node status (in a real scenario, use actual node resource data)
        Ok(Response::new(NodeStatusResponse {
            node_id: "node_1".to_string(),
            available_ram: 4096,
            available_cpu: 80,
            available_bandwidth: 50,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let node_service = MyNodeService::default();

    println!("Node gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(NodeServiceServer::new(node_service))
        .serve(addr)
        .await?;

    Ok(())
}
