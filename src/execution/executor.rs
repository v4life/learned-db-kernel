//! Query execution engine

use crate::error::Result;
use crate::execution::catalog::Catalog;
use crate::optimizer::planner::PhysicalPlan;

/// Query executor
pub struct QueryExecutor {
    pub catalog: Catalog,
}

impl QueryExecutor {
    pub fn new(catalog: Catalog) -> Self {
        QueryExecutor { catalog }
    }

    /// Execute a query plan
    pub fn execute(&self, _plan: &PhysicalPlan) -> Result<Vec<Vec<String>>> {
        // Simplified: return empty results
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let catalog = Catalog::new();
        let executor = QueryExecutor::new(catalog);
        assert_eq!(executor.catalog.tables.len(), 0);
    }
}
