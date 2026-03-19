use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("node not found: {0}")]
    NodeNotFound(u64),
    #[error("edge not found: {0} -> {1}")]
    EdgeNotFound(u64, u64),
    #[error("duplicate node: {0}")]
    DuplicateNode(u64),
}

/// In-memory graph storage engine.
pub struct GraphStore {
    nodes: BTreeMap<u64, NodeData>,
    edges: BTreeMap<u64, Vec<Edge>>,
}

#[derive(Clone, Debug)]
pub struct NodeData {
    pub id: u64,
    pub label: String,
    pub properties: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub from: u64,
    pub to: u64,
    pub label: String,
}

impl GraphStore {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
        }
    }

    pub fn add_node(&mut self, data: NodeData) -> Result<(), StorageError> {
        if self.nodes.contains_key(&data.id) {
            return Err(StorageError::DuplicateNode(data.id));
        }
        self.nodes.insert(data.id, data);
        Ok(())
    }

    pub fn get_node(&self, id: u64) -> Result<&NodeData, StorageError> {
        self.nodes.get(&id).ok_or(StorageError::NodeNotFound(id))
    }

    /// BUG: When removing a node, edges pointing TO this node from other nodes
    /// are not cleaned up. Only outgoing edges are removed.
    /// See issue #2.
    pub fn remove_node(&mut self, id: u64) -> Result<NodeData, StorageError> {
        let node = self.nodes.remove(&id).ok_or(StorageError::NodeNotFound(id))?;
        self.edges.remove(&id);
        Ok(node)
    }

    pub fn add_edge(&mut self, edge: Edge) -> Result<(), StorageError> {
        if !self.nodes.contains_key(&edge.from) {
            return Err(StorageError::NodeNotFound(edge.from));
        }
        if !self.nodes.contains_key(&edge.to) {
            return Err(StorageError::NodeNotFound(edge.to));
        }
        self.edges.entry(edge.from).or_default().push(edge);
        Ok(())
    }

    pub fn neighbors(&self, id: u64) -> Result<Vec<&Edge>, StorageError> {
        if !self.nodes.contains_key(&id) {
            return Err(StorageError::NodeNotFound(id));
        }
        Ok(self.edges.get(&id).map(|e| e.iter().collect()).unwrap_or_default())
    }

    // TODO: implement bulk insert for batch graph loading
    // Tracked in issue #5 — waiting on serialization format decision

    // TODO: add edge weight support
    // Deferred to v4.0 — needs schema migration plan

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|v| v.len()).sum()
    }
}
