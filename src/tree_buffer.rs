use crate::quat::{Quat, QuatByte};

/// High-performance, data-oriented buffer that stores the state of thousands
/// of UI nodes densely packed into `QuatByte` chunks.
pub struct QuatTreeBuffer {
    bytes: Vec<QuatByte>,
    capacity_nodes: usize,
}

impl QuatTreeBuffer {
    /// Creates a new tree buffer supporting a given maximum number of nodes.
    pub fn new(max_nodes: usize) -> Self {
        // Svaki QuatByte drži 4 čvora (4 * 2 bita = 8 bita)
        let byte_count = (max_nodes + 3) / 4;
        Self {
            bytes: vec![QuatByte::new(); byte_count],
            capacity_nodes: max_nodes,
        }
    }

    /// Sets the quaternary state of a specific node index efficiently.
    pub fn set_node_state(&mut self, node_index: usize, quat: Quat) -> Result<(), &'static str> {
        if node_index >= self.capacity_nodes {
            return Err("Node index out of buffer capacity bounds");
        }
        let byte_idx = node_index / 4;
        let sub_idx = node_index % 4;
        self.bytes[byte_idx].set(sub_idx, quat)
    }

    pub fn count_nodes_by_state(&self, target_quat: Quat) -> usize {
    let target_bits = target_quat.to_bits();
    let mut count = 0;

    // Iterate over packed u8 chunks in memory (Cache locality friendly)
    for byte in &self.bytes {
        // Unroll the 4 quats packed inside QuatByte
        for i in 0..4 {
            if let Some(q) = byte.get(i) {
                if q.to_bits() == target_bits {
                    count += 1;
                }
            }
        }
    }
    count
}

    /// Retrieves the quaternary state of a specific node index.
    pub fn get_node_state(&self, node_index: usize) -> Option<Quat> {
        if node_index >= self.capacity_nodes {
            return None;
        }
        let byte_idx = node_index / 4;
        let sub_idx = node_index % 4;
        self.bytes[byte_idx].get(sub_idx)
    }

    /// Returns the total memory footprint in bytes used by this UI node buffer.
    pub fn memory_usage_bytes(&self) -> usize {
        self.bytes.len() * std::mem::size_of::<QuatByte>()
    }
}