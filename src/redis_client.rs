use redis::AsyncCommands;

pub struct RedisClient {
    client: redis::Client,
}

impl RedisClient {
    pub async fn new() -> Self {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        Self { client }
    }

    // Store node status
    pub async fn set_node_status(
        &self,
        node_id: String,
        ram: u64,
        cpu: u64,
        bandwidth: u64,
    ) {
        let mut conn = self.client.get_async_connection().await.unwrap();
        let _: () = conn.hset_multiple(
            format!("node_status:{}", node_id),
            &[
                ("available_ram", ram.to_string()),
                ("available_cpu", cpu.to_string()),
                ("available_bandwidth", bandwidth.to_string()),
            ],
        )
        .await
        .unwrap();
    }

    // Get node status
    pub async fn get_node_status(&self, node_id: String) -> Option<(u64, u64, u64)> {
        let mut conn = self.client.get_async_connection().await.unwrap();
        let ram: u64 = conn.hget(format!("node_status:{}", node_id), "available_ram").await.unwrap();
        let cpu: u64 = conn.hget(format!("node_status:{}", node_id), "available_cpu").await.unwrap();
        let bandwidth: u64 = conn.hget(format!("node_status:{}", node_id), "available_bandwidth").await.unwrap();
        Some((ram, cpu, bandwidth))
    }

    // Store task assignment
    pub async fn assign_task_to_node(&self, task_id: String, node_id: String) {
        let mut conn = self.client.get_async_connection().await.unwrap();
        let _: () = conn.set(format!("task_assignment:{}", task_id), node_id).await.unwrap();
    }

    // Get task assignment
    pub async fn get_task_assignment(&self, task_id: String) -> Option<String> {
        let mut conn = self.client.get_async_connection().await.unwrap();
        conn.get(format!("task_assignment:{}", task_id)).await.unwrap()
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
