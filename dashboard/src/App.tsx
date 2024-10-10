import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [nodes, setNodes] = useState([]);
  const [tasks, setTasks] = useState([]);

  // Fetch node and task status from backend
  useEffect(() => {
    async function fetchStatus() {
      const nodesData = await invoke("get_node_status");
      const tasksData = await invoke("get_task_status");
      setNodes(nodesData);
      setTasks(tasksData);
    }

    const interval = setInterval(fetchStatus, 5000); // Poll every 5 seconds
    return () => clearInterval(interval);
  }, []);

  return (
    <div>
      <h1>Distributed System Dashboard</h1>

      <h2>Active Nodes</h2>
      <ul>
        {nodes.map(node => (
          <li key={node.node_id}>
            {node.node_id}: RAM: {node.allocated_ram}/{node.available_ram} MB, CPU: {node.allocated_cpu}/{node.available_cpu}%
          </li>
        ))}
      </ul>

      <h2>Tasks</h2>
      <ul>
        {tasks.map(task => (
          <li key={task.task_id}>
            Task {task.task_id}: {task.status}, Node: {task.assigned_node_id}
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
