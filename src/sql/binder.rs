//! Semantic analysis and binding

use crate::execution::catalog::Catalog;
use crate::sql::parser::SQLStatement;
use crate::error::Result;

/// Binder for semantic analysis
pub struct Binder {
    pub catalog: Catalog,
}

impl Binder {
    pub fn new(catalog: Catalog) -> Self {
        Binder { catalog }
    }

    /// Bind a SQL statement against the catalog
    pub fn bind(&self, _stmt: &SQLStatement) -> Result<()> {
        // Check that tables and columns exist in catalog
        // Resolve data types
        // Check constraints
        Ok(())
    }
}
