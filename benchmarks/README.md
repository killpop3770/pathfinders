# Performance Benchmarks

This directory contains performance measurements comparing different versions of Pathfinders Visualizer.

## Methodology

- **Tool**: Valgrind Massif (heap profiler)
- **Build**: `cargo build --release`
- **Test scenario**: Run BFS algorithm on 25x25 grid with 25% obstacles
- **Metrics**: Peak memory usage, heap allocation, execution time

## Files

- `v0.2.0_massif.out` - Baseline measurements (before architecture refactor)
- `v1.0.0_massif.out` - Current version (after optimizations)
- `results.md` - Detailed comparison and analysis

## How to reproduce

```bash
# Compile release version
cargo build --release

# Run with Massif
valgrind --tool=massif --massif-out-file=./benchmarks/v1.0.0_massif.out ./target/release/pathfinders

# View results
ms_print ./benchmarks/v1.0.0_massif.out
