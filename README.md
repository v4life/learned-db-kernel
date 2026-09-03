# Learned Database Kernel

A production-grade database kernel written in Rust, integrating machine learning models for indexing, query optimization, and buffer management. This project demonstrates a complete learned database system with hybrid indexing, adaptive query optimization, and intelligent storage management.

## Architecture Overview

### Core Layers

1. **Storage Layer** (`src/storage/`)
   - Page-based memory management with slotted pages
   - Direct I/O disk manager (io_uring support)
   - Learned buffer pool with predictive prefetching
   - MVCC transaction support

2. **Index Layer** (`src/index/`)
   - Recursive Model Index (RMI) for multi-stage prediction
   - Piecewise Geometric Model (PGM) Index
   - Hybrid nodes that dynamically switch between learned models and B+Tree
   - Bounded error search with SIMD acceleration

3. **Optimizer Layer** (`src/optimizer/`)
   - Neural network-based cardinality estimation
   - ML-driven cost modeling
   - Learned query plan generation
   - Join reordering optimization

4. **Execution Layer** (`src/execution/`)
   - Schema catalog and metadata management
   - Vectorized query operators (Scan, Filter, Join, Aggregate)
   - Volcano-style query execution
   - Transaction manager with learned lock scheduling

5. **Compute Utilities** (`src/compute/`)
   - Fast online linear regression (OLS/Splines)
   - SIMD-accelerated search operations
   - Mathematical acceleration primitives

## Project Structure

```
learned-db-kernel/
├── Cargo.toml                    # Workspace dependencies
├── README.md                     # This file
├── benches/                      # Performance benchmarks
│   ├── index_bench.rs            # RMI vs. PGM vs. BTree
│   └── optimizer_bench.rs        # Cardinality estimation latency
├── src/
│   ├── main.rs                   # CLI entry point
│   ├── lib.rs                    # Crate exports
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── page.rs               # Slotted-page layout
│   │   ├── disk_manager.rs       # I/O abstraction
│   │   └── buffer_pool.rs        # Learned buffer management
│   ├── index/
│   │   ├── mod.rs
│   │   ├── rmi.rs                # Recursive Model Index
│   │   ├── pgm.rs                # Piecewise Geometric Model
│   │   ├── fallback_btree.rs     # B+Tree fallback
│   │   └── hybrid_node.rs        # Dynamic hybrid nodes
│   ├── optimizer/
│   │   ├── mod.rs
│   │   ├── cardinality.rs        # Learned cardinality estimation
│   │   ├── cost_model.rs         # ML-based cost estimation
│   │   └── plan_generator.rs     # Query plan generation
│   ├── execution/
│   │   ├── mod.rs
│   │   ├── catalog.rs            # Schema metadata
│   │   ├── operators.rs          # Query operators
│   │   └── transaction.rs        # MVCC transaction manager
│   └── compute/
│       ├── mod.rs
│       ├── regression.rs         # Linear regression fitting
│       └── simd_search.rs        # SIMD search acceleration
└── tests/
    ├── storage_tests.rs
    └── query_tests.rs
```

## Key Features

- **Learned Indexing**: Adaptive indexes that predict key locations using trained models
- **Hybrid Structures**: Dynamic switching between learned models and traditional indexes
- **Predictive Prefetching**: Buffer pool anticipates access patterns and preemptively loads pages
- **ML-Driven Optimization**: Neural networks estimate selectivity and join costs
- **Vectorized Execution**: Efficient columnar query processing with SIMD acceleration
- **MVCC Transactions**: Lock-free snapshot isolation with learned lock scheduling

## Building

```bash
cargo build --release
```

## Running Benchmarks

```bash
# Index performance comparison
cargo bench --bench index_bench

# Optimizer latency benchmarks
cargo bench --bench optimizer_bench
```

## Running Tests

```bash
cargo test
```

## Development Status

- [x] Core storage layer with paging
- [x] Buffer pool with predictive prefetch
- [x] Hybrid index structures
- [x] Basic cardinality estimator
- [ ] Full query execution engine
- [ ] Transaction system
- [ ] Distributed query processing
- [ ] Advanced ML models

## Performance Goals

- **Index Lookup**: O(1) average case with learned models
- **Buffer Hit Rate**: 95%+ with predictive prefetching
- **Query Planning**: <10ms for complex queries
- **Throughput**: 100K+ ops/sec on commodity hardware

## References

- [Learned Index Structures](https://arxiv.org/abs/1712.01208)
- [The Case for Learned Index Structures](https://arxiv.org/abs/1712.01208)
- [PGM-Index: A Scalable Approximate Index](https://arxiv.org/abs/1910.06169)
- [Machine Learning for Systems and Systems for Machine Learning](https://arxiv.org/abs/1905.13328)

## License

MIT

## Authors

- v4life
