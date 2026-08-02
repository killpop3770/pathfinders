# Performance Comparison: v0.2.0 → v1.0.0

## Summary

After major architecture refactor and performance optimizations, Pathfinders Visualizer shows significant improvements:

- **Memory usage**: -11.9% (1.3 MB saved)
- **Execution time**: -73% (3.7× faster)
- **Heap efficiency**: -12.0% less fragmentation

## Detailed Results

### Memory Consumption (Peak)

| Version | Total Memory | Useful Heap | Extra Heap | Improvement |
|---------|--------------|-------------|------------|-------------|
| v0.2.0  | 11.47 MB     | 11.35 MB    | 126.9 KB   | baseline    |
| v1.0.0  | 10.11 MB     | 9.99 MB     | 115.5 KB   | **-11.9%**  |

### Execution Time

| Version | Time (seconds) | Improvement |
|---------|----------------|-------------|
| v0.2.0  | ~11.0          | baseline    |
| v1.0.0  | ~3.0           | **-73%**    |

## What Changed

### Optimizations Applied

1. **Flat vector representation**
   - Replaced `Vec<Vec<Cell>>` with `Vec<Cell>`
   - Improved cache locality for grid traversal
   - Reduced memory fragmentation

2. **Mutex optimization**
   - Reduced lock contention from 3-4 per neighbor to 1 per batch
   - Moved `state.wait()` outside lock blocks
   - Eliminated UI thread blocking

3. **Removed redundant data**
   - Eliminated unused `CellCoordinates` field (saved 4 bytes per cell)
   - For 25×25 grid: 2,500 bytes saved

4. **Architecture improvements**
   - Separated rendering logic from business logic
   - Reduced unnecessary cloning and allocations

## Raw Data

See `v0.2.0_massif.out` and `v1.0.0_massif.out` for complete Massif output.

## Conclusion

The refactoring achieved **significant performance gains** while improving code maintainability and architecture. Memory usage decreased by ~12%, and execution time improved by ~73%, making the visualizer more responsive and efficient.