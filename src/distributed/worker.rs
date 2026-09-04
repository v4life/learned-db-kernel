//! Worker node implementation for distributed execution

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Task for worker to execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub task_type: String,
    pub partition_id: u32,
    pub query_id: String,
    pub payload: Vec<u8>,
}

/// Task result from worker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: u64,
    pub worker_id: u32,
    pub status: TaskStatus,
    pub rows_processed: usize,
    pub execution_time_ms: u64,
    pub result_data: Vec<u8>,
}

/// Task execution status
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Worker node that executes distributed tasks
pub struct WorkerNode {
    pub id: u32,
    pub task_queue: VecDeque<Task>,
    pub completed_tasks: Vec<TaskResult>,
    pub capacity: usize, // Max concurrent tasks
}

impl WorkerNode {
    pub fn new(id: u32, capacity: usize) -> Self {
        WorkerNode {
            id,
            task_queue: VecDeque::new(),
            completed_tasks: Vec::new(),
            capacity,
        }
    }

    /// Enqueue a task for execution
    pub fn enqueue_task(&mut self, task: Task) -> bool {
        if self.task_queue.len() < self.capacity {
            self.task_queue.push_back(task);
            true
        } else {
            false // Queue full
        }
    }

    /// Get next task to execute
    pub fn next_task(&mut self) -> Option<Task> {
        self.task_queue.pop_front()
    }

    /// Record completed task
    pub fn record_result(&mut self, result: TaskResult) {
        self.completed_tasks.push(result);
    }

    /// Get queue depth
    pub fn queue_depth(&self) -> usize {
        self.task_queue.len()
    }

    /// Get utilization percentage
    pub fn utilization(&self) -> f64 {
        self.task_queue.len() as f64 / self.capacity as f64 * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_creation() {
        let worker = WorkerNode::new(1, 10);
        assert_eq!(worker.id, 1);
        assert_eq!(worker.capacity, 10);
        assert_eq!(worker.queue_depth(), 0);
    }

    #[test]
    fn test_task_enqueue() {
        let mut worker = WorkerNode::new(1, 2);

        let task = Task {
            id: 1,
            task_type: "scan".to_string(),
            partition_id: 0,
            query_id: "q1".to_string(),
            payload: vec![],
        };

        assert!(worker.enqueue_task(task));
        assert_eq!(worker.queue_depth(), 1);
    }

    #[test]
    fn test_queue_full() {
        let mut worker = WorkerNode::new(1, 1);

        let task1 = Task {
            id: 1,
            task_type: "scan".to_string(),
            partition_id: 0,
            query_id: "q1".to_string(),
            payload: vec![],
        };

        let task2 = Task {
            id: 2,
            task_type: "filter".to_string(),
            partition_id: 0,
            query_id: "q1".to_string(),
            payload: vec![],
        };

        assert!(worker.enqueue_task(task1));
        assert!(!worker.enqueue_task(task2)); // Should fail
    }
}
