//! Learned cost model for query optimization
//!
//! Predicts execution cost of query plans using trained models.

use serde::{Deserialize, Serialize};

/// Query operator type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OperatorType {
    TableScan,
    IndexScan,
    Filter,
    HashJoin,
    SortMergeJoin,
    NestedLoopJoin,
    Aggregate,
    Sort,
    Limit,
}

/// Operator cost parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorCost {
    pub operator_type: OperatorType,
    pub input_rows: usize,
    pub output_rows: usize,
    pub selectivity: f64,
}

impl OperatorCost {
    pub fn new(
        operator_type: OperatorType,
        input_rows: usize,
        output_rows: usize,
        selectivity: f64,
    ) -> Self {
        OperatorCost {
            operator_type,
            input_rows,
            output_rows,
            selectivity,
        }
    }
}

/// Learned cost model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostModel {
    /// Learned coefficients for each operator type
    pub operator_costs: std::collections::HashMap<u32, f64>,
    /// I/O cost per page
    pub io_cost: f64,
    /// CPU cost per row
    pub cpu_cost: f64,
}

impl CostModel {
    pub fn new() -> Self {
        let mut operator_costs = std::collections::HashMap::new();
        operator_costs.insert(OperatorType::TableScan as u32, 1.0);
        operator_costs.insert(OperatorType::IndexScan as u32, 0.1);
        operator_costs.insert(OperatorType::Filter as u32, 0.5);
        operator_costs.insert(OperatorType::HashJoin as u32, 2.0);
        operator_costs.insert(OperatorType::SortMergeJoin as u32, 2.5);
        operator_costs.insert(OperatorType::NestedLoopJoin as u32, 5.0);
        operator_costs.insert(OperatorType::Aggregate as u32, 1.5);
        operator_costs.insert(OperatorType::Sort as u32, 3.0);
        operator_costs.insert(OperatorType::Limit as u32, 0.1);

        CostModel {
            operator_costs,
            io_cost: 10.0,  // milliseconds per page
            cpu_cost: 0.01, // milliseconds per row
        }
    }

    /// Estimate cost of an operator
    pub fn estimate_cost(&self, operator: &OperatorCost) -> f64 {
        let base_cost = *self
            .operator_costs
            .get(&(operator.operator_type as u32))
            .unwrap_or(&1.0);

        let row_cost = operator.output_rows as f64 * self.cpu_cost;
        let io_cost = (operator.output_rows as f64 / 100.0) * self.io_cost; // Assume 100 rows per page

        base_cost * (row_cost + io_cost)
    }

    /// Estimate total cost of a plan (operators in sequence)
    pub fn estimate_total_cost(&self, operators: &[OperatorCost]) -> f64 {
        operators.iter().map(|op| self.estimate_cost(op)).sum()
    }

    /// Train on observed query execution times
    pub fn train(&mut self, _observations: &[(Vec<OperatorCost>, f64)]) {
        // In production, use regression to update operator costs
        // For now, use hardcoded values
    }
}

impl Default for CostModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_model_creation() {
        let model = CostModel::new();
        assert_eq!(model.io_cost, 10.0);
        assert_eq!(model.cpu_cost, 0.01);
    }

    #[test]
    fn test_operator_cost_estimation() {
        let model = CostModel::new();
        let op = OperatorCost::new(OperatorType::TableScan, 1000, 500, 0.5);
        let cost = model.estimate_cost(&op);
        assert!(cost > 0.0);
    }

    #[test]
    fn test_plan_cost_comparison() {
        let model = CostModel::new();

        let plan_a = vec![
            OperatorCost::new(OperatorType::TableScan, 1000, 1000, 1.0),
            OperatorCost::new(OperatorType::Filter, 1000, 500, 0.5),
        ];

        let plan_b = vec![
            OperatorCost::new(OperatorType::IndexScan, 1000, 500, 0.5),
        ];

        let cost_a = model.estimate_total_cost(&plan_a);
        let cost_b = model.estimate_total_cost(&plan_b);

        println!("Plan A cost: {}, Plan B cost: {}", cost_a, cost_b);
        // Index scan should be faster
        assert!(cost_b < cost_a);
    }
}
