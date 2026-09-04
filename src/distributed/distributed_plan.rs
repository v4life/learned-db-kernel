//! Distributed query plan representation

use serde::{Deserialize, Serialize};

/// Distributed query plan stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributedStage {
    /// Data scan across partitions
    PartitionedScan {
        table_id: u32,
        partitions: Vec<u32>,
    },
    /// Local filter on each partition
    LocalFilter {
        predicate: String,
    },
    /// Shuffle/repartition data
    Shuffle {
        key: String,
        target_partitions: Vec<u32>,
    },
    /// Aggregate across partitions
    Aggregate {
        group_by: Vec<String>,
        aggregates: Vec<String>,
    },
    /// Join operation
    DistributedJoin {
        left_table: u32,
        right_table: u32,
        join_key: String,
    },
}

/// Distributed query execution plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedPlan {
    pub query_id: String,
    pub stages: Vec<DistributedStage>,
    pub num_partitions: usize,
    pub estimated_cost: f64,
}

impl DistributedPlan {
    pub fn new(query_id: String, num_partitions: usize) -> Self {
        DistributedPlan {
            query_id,
            stages: Vec::new(),
            num_partitions,
            estimated_cost: 0.0,
        }
    }

    /// Add a stage to the plan
    pub fn add_stage(&mut self, stage: DistributedStage) {
        self.stages.push(stage);
    }

    /// Get execution parallelism
    pub fn parallelism(&self) -> usize {
        self.num_partitions
    }

    /// Estimate network bytes to shuffle
    pub fn estimate_shuffle_bytes(&self, avg_row_size: usize, total_rows: usize) -> usize {
        // Simplified: assume one shuffle per aggregate stage
        let shuffles = self.stages.iter().filter(|s| matches!(s, DistributedStage::Shuffle { .. })).count();
        shuffles * (total_rows / 10) * avg_row_size // Assume 10% of data shuffled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distributed_plan_creation() {
        let plan = DistributedPlan::new("q1".to_string(), 4);
        assert_eq!(plan.num_partitions, 4);
        assert_eq!(plan.parallelism(), 4);
    }

    #[test]
    fn test_add_stages() {
        let mut plan = DistributedPlan::new("q1".to_string(), 4);
        plan.add_stage(DistributedStage::PartitionedScan {
            table_id: 1,
            partitions: vec![0, 1, 2, 3],
        });

        assert_eq!(plan.stages.len(), 1);
    }
}
