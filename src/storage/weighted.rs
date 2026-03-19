/// Weighted edge support for graph algorithms.
pub struct WeightedEdge {
    pub from: u64,
    pub to: u64,
    pub label: String,
    pub weight: f64,
}

impl WeightedEdge {
    pub fn new(from: u64, to: u64, label: &str, weight: f64) -> Self {
        Self {
            from,
            to,
            label: label.to_string(),
            weight,
        }
    }
}
