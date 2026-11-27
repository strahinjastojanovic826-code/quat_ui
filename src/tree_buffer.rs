use crate::quat::{Quat, QuatByte};

/// High-performance, data-oriented buffer that stores the state of thousands
/// of UI nodes densely packed into `QuatByte` chunks.
pub struct QuatTreeBuffer {
    pub bytes: Vec<QuatByte>,
    pub capacity_nodes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuatSnapshot {
    pub capacity_nodes: usize,
    pub raw_bytes: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct QuatStateStats {
    pub total_nodes: usize,
    pub v00_count: usize,
    pub v01_count: usize,
    pub v10_count: usize,
    pub v11_count: usize,
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

    /// Returns an accessibility-friendly flat list of node states
    pub fn get_accessibility_tree(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for byte in &self.bytes {
            for i in 0..4 {
                if let Some(quat) = byte.get(i) {
                    result.push(quat.to_bits());
                }
            }
        }
        result
    }

    /// Returns the total memory footprint in bytes used by this UI node buffer.
    pub fn memory_usage_bytes(&self) -> usize {
        self.bytes.len() * std::mem::size_of::<QuatByte>()
    }

    /// Marks a single node index as dirty (changed since last render/sync).
    pub fn mark_dirty(&self, dirty_flags: &mut Vec<bool>, node_index: usize) -> Result<(), &'static str> {
        if node_index >= self.capacity_nodes {
            return Err("Node index out of buffer capacity bounds");
        }
        if dirty_flags.len() < self.capacity_nodes {
            dirty_flags.resize(self.capacity_nodes, false);
        }
        dirty_flags[node_index] = true;
        Ok(())
    }

    /// Sets node state and automatically marks `node_index` as dirty if the state changed.
    pub fn set_node_state_tracked(
        &mut self,
        node_index: usize,
        quat: Quat,
        dirty_flags: &mut Vec<bool>,
    ) -> Result<bool, &'static str> {
        let prev = self.get_node_state(node_index).ok_or("Node index out of buffer capacity bounds")?;
        self.set_node_state(node_index, quat)?;
        if dirty_flags.len() < self.capacity_nodes {
            dirty_flags.resize(self.capacity_nodes, false);
        }
        let changed = prev != quat;
        if changed {
            dirty_flags[node_index] = true;
        }
        Ok(changed)
    }

    /// Clears all dirty flags.
    pub fn clear_dirty_flags(&self, dirty_flags: &mut [bool]) {
        for flag in dirty_flags.iter_mut() {
            *flag = false;
        }
    }

    /// Sets all nodes in `[start, end)` to `quat`.
    /// Returns `Err` if `start > end` or `end > self.capacity_nodes`.
    pub fn set_range(
        &mut self,
        start: usize,
        end: usize,
        quat: Quat,
    ) -> Result<(), &'static str> {
        if start > end || end > self.capacity_nodes {
            return Err("Invalid range for QuatTreeBuffer::set_range");
        }
        for idx in start..end {
            self.set_node_state(idx, quat)?;
        }
        Ok(())
    }

    /// Resets all nodes up to `capacity_nodes` back to `Quat::V00`.
    pub fn clear_all(&mut self) {
        for byte in &mut self.bytes {
            byte.0 = 0;
        }
    }

    /// Returns all valid node indices (`0..self.capacity_nodes`) matching `target_quat`.
    pub fn indices_by_state(&self, target_quat: Quat) -> Vec<usize> {
        let target_bits = target_quat.to_bits();
        let mut matches = Vec::new();
        for idx in 0..self.capacity_nodes {
            if self.get_node_state(idx).map(|q| q.to_bits()) == Some(target_bits) {
                matches.push(idx);
            }
        }
        matches
    }

    /// Applies bitwise OR between corresponding node states of `self` and `other`
    /// up to `self.capacity_nodes.min(other.capacity_nodes)`.
    pub fn apply_bitwise_or(&mut self, other: &QuatTreeBuffer) {
        let limit = self.capacity_nodes.min(other.capacity_nodes);
        for idx in 0..limit {
            if let (Some(a), Some(b)) = (self.get_node_state(idx), other.get_node_state(idx)) {
                let merged = Quat::from_bits(a.to_bits() | b.to_bits());
                let _ = self.set_node_state(idx, merged);
            }
        }
    }

    /// Captures an immutable byte-level snapshot of the current quaternary states.
    pub fn snapshot(&self) -> QuatSnapshot {
        QuatSnapshot {
            capacity_nodes: self.capacity_nodes,
            raw_bytes: self.bytes.iter().map(|b| b.0).collect(),
        }
    }

    /// Restores buffer contents from a previously captured `QuatSnapshot`.
    pub fn restore_snapshot(&mut self, snap: &QuatSnapshot) -> Result<(), &'static str> {
        let expected_bytes = (snap.capacity_nodes + 3) / 4;
        if snap.raw_bytes.len() != expected_bytes {
            return Err("Snapshot byte length does not match capacity_nodes");
        }
        self.capacity_nodes = snap.capacity_nodes;
        self.bytes = snap.raw_bytes.iter().copied().map(QuatByte).collect();
        Ok(())
    }

    /// Computes distribution statistics across all four quaternary states (`V00`..`V11`).
    pub fn state_stats(&self) -> QuatStateStats {
        let mut stats = QuatStateStats {
            total_nodes: self.capacity_nodes,
            ..QuatStateStats::default()
        };

        for idx in 0..self.capacity_nodes {
            if let Some(q) = self.get_node_state(idx) {
                match q {
                    Quat::V00 => stats.v00_count += 1,
                    Quat::V01 => stats.v01_count += 1,
                    Quat::V10 => stats.v10_count += 1,
                    Quat::V11 => stats.v11_count += 1,
                }
            }
        }

        stats
    }

    /// Returns a compact human-readable debug summary of buffer state counts.
    pub fn debug_summary(&self) -> String {
        let s = self.state_stats();
        format!(
            "QuatTreeBuffer(nodes={}, bytes={}, V00={}, V01={}, V10={}, V11={})",
            s.total_nodes,
            self.memory_usage_bytes(),
            s.v00_count,
            s.v01_count,
            s.v10_count,
            s.v11_count
        )
    }

    /// Fills all capacity nodes with a single quaternary state.
    pub fn fill(&mut self, quat: Quat) {
        let bits = quat.to_bits();
        let packed = (bits << 6) | (bits << 4) | (bits << 2) | bits;
        for byte in &mut self.bytes {
            byte.0 = packed;
        }
    }

    /// Counts nodes within `[start, end)` matching `target_quat`.
    pub fn count_in_range(&self, start: usize, end: usize, target_quat: Quat) -> usize {
        let clamped_end = end.min(self.capacity_nodes);
        if start >= clamped_end {
            return 0;
        }
        let target_bits = target_quat.to_bits();
        (start..clamped_end)
            .filter(|&idx| self.get_node_state(idx).map(|q| q.to_bits()) == Some(target_bits))
            .count()
    }

    /// Returns indices of nodes whose quaternary state differs from `other`.
    pub fn diff_indices(&self, other: &QuatTreeBuffer) -> Vec<usize> {
        let len = self.capacity_nodes.min(other.capacity_nodes);
        let mut changed = Vec::new();
        for idx in 0..len {
            if self.get_node_state(idx) != other.get_node_state(idx) {
                changed.push(idx);
            }
        }
        changed
    }

}