# Hybrid Learned Index Kernel

A high-performance storage engine kernel combining piecewise linear regression models (RMI/PGM) with dynamic fallback structures (B+ Trees / Gapped Arrays).

## Features
- **Top-tier routing:** O(1) model-based key range prediction.
- **Fallback safety:** Bounded binary search on model miss.
- **Dynamic writes:** Gapped array strategy for continuous updates.
