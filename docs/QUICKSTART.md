# Quickstart

```rust
use fastgraph::storage::{GraphStore, NodeData, Edge};
use std::collections::HashMap;

let mut store = GraphStore::new();

// Add nodes
store.add_node(NodeData {
    id: 1,
    label: "Person".to_string(),
    properties: HashMap::from([("name".to_string(), "Alice".to_string())]),
}).unwrap();

// Add edges
store.add_edge(Edge {
    from: 1,
    to: 2,
    label: "KNOWS".to_string(),
}).unwrap();
```
