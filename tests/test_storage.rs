use fastgraph::storage::{Edge, GraphStore, NodeData, StorageError};
use std::collections::HashMap;

#[test]
fn test_add_and_get_node() {
    let mut store = GraphStore::new();
    let node = NodeData {
        id: 1,
        label: "Person".to_string(),
        properties: HashMap::new(),
    };
    store.add_node(node).unwrap();
    let result = store.get_node(1).unwrap();
    assert_eq!(result.label, "Person");
}

#[test]
fn test_duplicate_node() {
    let mut store = GraphStore::new();
    let node = NodeData {
        id: 1,
        label: "Person".to_string(),
        properties: HashMap::new(),
    };
    store.add_node(node.clone()).unwrap();
    assert!(matches!(
        store.add_node(node),
        Err(StorageError::DuplicateNode(1))
    ));
}

#[test]
fn test_add_edge_and_neighbors() {
    let mut store = GraphStore::new();
    store
        .add_node(NodeData {
            id: 1,
            label: "A".to_string(),
            properties: HashMap::new(),
        })
        .unwrap();
    store
        .add_node(NodeData {
            id: 2,
            label: "B".to_string(),
            properties: HashMap::new(),
        })
        .unwrap();
    store
        .add_edge(Edge {
            from: 1,
            to: 2,
            label: "KNOWS".to_string(),
        })
        .unwrap();
    let neighbors = store.neighbors(1).unwrap();
    assert_eq!(neighbors.len(), 1);
    assert_eq!(neighbors[0].to, 2);
}

#[test]
fn test_remove_node() {
    let mut store = GraphStore::new();
    store
        .add_node(NodeData {
            id: 1,
            label: "A".to_string(),
            properties: HashMap::new(),
        })
        .unwrap();
    let removed = store.remove_node(1).unwrap();
    assert_eq!(removed.label, "A");
    assert!(matches!(
        store.get_node(1),
        Err(StorageError::NodeNotFound(1))
    ));
}
