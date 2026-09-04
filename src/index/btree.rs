//! B+ Tree implementation as fallback index

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// B+ Tree node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BTreeNode {
    pub keys: Vec<f64>,
    pub values: Vec<usize>,
    pub is_leaf: bool,
}

impl BTreeNode {
    pub fn new(is_leaf: bool) -> Self {
        BTreeNode {
            keys: Vec::new(),
            values: Vec::new(),
            is_leaf,
        }
    }
}

/// B+ Tree implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BTree {
    pub root: BTreeNode,
    pub order: usize,
}

impl BTree {
    pub fn new(order: usize) -> Self {
        BTree {
            root: BTreeNode::new(true),
            order,
        }
    }

    /// Search for a key in the B+ tree
    pub fn search(&self, key: f64) -> Option<usize> {
        self.search_node(&self.root, key)
    }

    fn search_node(&self, node: &BTreeNode, key: f64) -> Option<usize> {
        match node.keys.binary_search_by(|k| k.partial_cmp(&key).unwrap()) {
            Ok(idx) => Some(node.values[idx]),
            Err(_) => None,
        }
    }

    /// Insert a key-value pair
    pub fn insert(&mut self, key: f64, value: usize) -> Result<()> {
        // Simplified insert: just add to root
        let idx = self.root.keys.binary_search_by(|k| k.partial_cmp(&key).unwrap());
        match idx {
            Ok(pos) => self.root.values[pos] = value,
            Err(pos) => {
                self.root.keys.insert(pos, key);
                self.root.values.insert(pos, value);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btree_insert_search() -> Result<()> {
        let mut btree = BTree::new(3);
        btree.insert(5.0, 10)?;
        btree.insert(3.0, 6)?;

        assert_eq!(btree.search(5.0), Some(10));
        assert_eq!(btree.search(3.0), Some(6));
        Ok(())
    }
}
