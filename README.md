# @affectively/wasm-graph-algorithms

High-performance WebAssembly graph algorithms written in Rust.

[![npm version](https://img.shields.io/npm/v/@affectively/wasm-graph-algorithms.svg)](https://www.npmjs.com/package/@affectively/wasm-graph-algorithms)
[![crates.io](https://img.shields.io/crates/v/affectively-graph-algorithms.svg)](https://crates.io/crates/affectively-graph-algorithms)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Cycle Detection** - Detect cycles in directed graphs
- **DAG Operations** - Directed Acyclic Graph utilities
- **Path Finding** - Shortest path algorithms (Dijkstra, A*)
- **Topological Sort** - Order nodes by dependencies

## Installation

```bash
npm install @affectively/wasm-graph-algorithms
```

## Quick Start

```typescript
import init, { detect_cycles, find_path, topological_sort } from '@affectively/wasm-graph-algorithms';

await init();

// Cycle detection
const hasCycle = detect_cycles(graph);

// Path finding
const path = find_path(graph, start, end);

// Topological sort
const sorted = topological_sort(dag);
```

## License

MIT License - see [LICENSE](./LICENSE) for details.

---

Made with ️ by [AFFECTIVELY](https://affectively.ai)
