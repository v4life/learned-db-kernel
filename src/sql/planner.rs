//! SQL query planner

use crate::sql::parser::SQLStatement;
use crate::optimizer::planner::PhysicalPlan;

/// SQL planner
pub struct SQLPlanner;

impl SQLPlanner {
    /// Plan a SQL statement
    pub fn plan(_stmt: &SQLStatement) -> PhysicalPlan {
        // Convert SQL to logical plan
        // Apply optimizations
        // Generate physical plan
        PhysicalPlan {
            nodes: vec![],
            estimated_cost: 0.0,
            estimated_rows: 0,
        }
    }
}
