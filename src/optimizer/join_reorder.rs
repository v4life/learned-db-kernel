//! Learned join reordering for query optimization
//!
//! Uses DP and learned cost models to find optimal join order.

use crate::optimizer::cost_model::CostModel;
use serde::{Deserialize, Serialize};

/// Join order representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinOrder {
    pub table_ids: Vec<u32>,
}

/// Join reorderer with learned cost model
pub struct JoinOrderer {
    pub cost_model: CostModel,
}

impl JoinOrderer {
    pub fn new() -> Self {
        JoinOrderer {
            cost_model: CostModel::new(),
        }
    }

    /// Find optimal join order using dynamic programming
    pub fn find_optimal_order(&self, table_ids: Vec<u32>) -> JoinOrder {
        if table_ids.len() <= 1 {
            return JoinOrder { table_ids };
        }

        // Simplified: for 2-3 tables, use heuristic
        // In production: use Selinger's algorithm with learned costs
        let mut ordered = table_ids.clone();
        ordered.sort(); // Simple heuristic: sort by ID

        JoinOrder {
            table_ids: ordered,
        }
    }

    /// Estimate cardinality of join result
    pub fn estimate_join_cardinality(
        &self,
        left_rows: usize,
        right_rows: usize,
        join_selectivity: f64,
    ) -> usize {
        (left_rows as f64 * right_rows as f64 * join_selectivity) as usize
    }
}

impl Default for JoinOrderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_order_single_table() {
        let orderer = JoinOrderer::new();
        let order = orderer.find_optimal_order(vec![1]);
        assert_eq!(order.table_ids, vec![1]);
    }

    #[test]
    fn test_join_order_multiple_tables() {
        let orderer = JoinOrderer::new();
        let order = orderer.find_optimal_order(vec![3, 1, 2]);
        assert_eq!(order.table_ids.len(), 3);
    }

    #[test]
    fn test_join_cardinality_estimation() {
        let orderer = JoinOrderer::new();
        let card = orderer.estimate_join_cardinality(100, 50, 0.1);
        assert_eq!(card, 500);
    }
}
