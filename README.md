# Distributed Computing System

## Overview

This project implements a **distributed computing system** that allows individual users to run nodes on their devices/mini servers/mini daacenters and sale resources such as **CPU**, **RAM**, and **storage** across multiple nodes in a network. The system supports **task scheduling**, **task migration**, **load balancing**, and **resource sharing**. 

Users can configure the amount of RAM and storage to be sold, and the platform dynamically manages resource allocation.

The system integrates the **InterPlanetary File System (IPFS)** for distributed storage, **WebSocket** and **gRPC** for communication between nodes, and an **ML-based load prediction engine** for optimized task scheduling.

## Features

- **Node Management**: Users can activate and manage nodes that participate in the distributed system. The system monitors each node’s resource usage.

- **Resource Management**: Dynamic CPU and RAM allocation, resource pooling, and adjustments based on load predictions.

- **Task Scheduling**: Priority-based task scheduling, deadline-aware scheduling, and dynamic task preemption.

- **Task Migration**: Tasks can be migrated to other nodes based on system load, resource availability, and network conditions.

- **Communication Layer**: WebSocket and gRPC protocols for real-time communication between nodes.

- **ML-Based Prediction**: A machine learning engine predicts CPU and RAM loads to optimize task scheduling.

- **IPFS Storage**: Supports distributed file storage using the IPFS protocol.

Installation and Setup

Prerequisites

Rust: 

Ensure you have Rust installed on your system.

IPFS: 

Install IPFS to support distributed file storage.

Tonic (for gRPC): 

Ensure you have the dependencies for gRPC installed.

Clone the Repository

bash

git clone https://github.com/your-repo/distributed-computing-system.git

cd distributed-computing-system

Install Dependencies

Install the necessary Rust dependencies by running:

bash

cargo build

Configuration

Set up the system configuration in config/config.toml. Here’s an example:

toml

[network]

grpc_port = "50051"

websocket_port = "9001"

[ipfs]

api_url = "http://localhost:5001"

Running the System

Run the application using:

bash

cargo run

This will start the distributed computing system, including the node management, task scheduling, and communication layers.

Testing

You can run the tests for each module using:

bash

cargo test

This will execute the tests for various modules, including node management, resource management, task scheduling, and communication.

Module Descriptions

1. main.rs

The entry point of the application. Initializes the system and manages the overall flow of node activation, resource allocation, task scheduling, and communication between nodes.

2. user_manager.rs

Handles user authentication and session management using OAuth2. Users can log in and manage their sessions securely.

3. node_manager.rs

Manages the node lifecycle (activation, deactivation, and monitoring). This module tracks each node's resource availability and current load.

4. resource_manager.rs

Responsible for resource allocation (CPU, RAM) and dynamic adjustments based on task load. Also manages resource pooling across nodes.

5. ipfs_storage_manager.rs

Integrates with the IPFS protocol to provide distributed storage across nodes. Handles file uploads, retrieval, and storage management.

6. communication.rs

Implements WebSocket and gRPC communication between nodes, allowing nodes to exchange resource information, task migration requests, and status updates in real-time.

7. task_scheduler.rs

Handles task scheduling based on task priority, deadlines, and resource availability. Supports task migration and preemption for efficient resource usage.

8. prediction.rs

An ML-based prediction engine that uses an LSTM model to predict CPU and RAM usage. This helps optimize task scheduling by accounting for future resource demands.

9. escalation.rs

Implements dynamic escalation thresholds for task prioritization, ensuring that high-priority tasks are escalated during periods of high load.

10. utils.rs

Provides utility functions such as logging and other helper methods used throughout the system.

Contribution Guidelines

Feel free to contribute to this project by submitting pull requests or creating issues for bug reports and feature requests. Contributions to improve the scheduling, migration, and resource management algorithms are welcome!

License
This project is licensed under the MIT License.
