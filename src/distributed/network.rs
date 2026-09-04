//! Network communication layer for distributed execution

use serde::{Deserialize, Serialize};

/// Network message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    TaskSubmission { task_id: u64, payload: Vec<u8> },
    TaskResult { task_id: u64, result: Vec<u8> },
    Heartbeat { worker_id: u32 },
    WorkerRegister { worker_id: u32, host: String },
}

/// Network manager for inter-node communication
pub struct NetworkManager {
    pub node_id: u32,
    pub connected_nodes: std::collections::HashMap<u32, String>,
}

impl NetworkManager {
    pub fn new(node_id: u32) -> Self {
        NetworkManager {
            node_id,
            connected_nodes: std::collections::HashMap::new(),
        }
    }

    /// Connect to a remote node
    pub fn connect(&mut self, node_id: u32, address: String) {
        self.connected_nodes.insert(node_id, address);
    }

    /// Send message to a node (simplified)
    pub fn send_message(&self, to_node: u32, _message: &NetworkMessage) -> bool {
        // In production: implement actual network I/O
        self.connected_nodes.contains_key(&to_node)
    }

    /// Get connection status
    pub fn is_connected(&self, node_id: u32) -> bool {
        self.connected_nodes.contains_key(&node_id)
    }

    /// Get list of connected nodes
    pub fn get_connected_nodes(&self) -> Vec<u32> {
        self.connected_nodes.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_manager() {
        let mut nm = NetworkManager::new(1);
        nm.connect(2, "localhost:5433".to_string());

        assert!(nm.is_connected(2));
        assert!(!nm.is_connected(3));
    }
}
