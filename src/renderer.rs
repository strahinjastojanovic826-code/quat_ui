use crate::tree_buffer::QuatTreeBuffer;
use crate::theme::Theme;

pub trait RendererBackend {
    fn begin_frame(&mut self);
    // Draw a node using its quaternary state and theme color
    fn draw_node(&mut self, index: usize, state_bits: u8, theme: &Theme);
    fn end_frame(&mut self);
}

// Primer kako se iterira kroz tvoj optimizovani bafer za renderovanje
pub fn render_tree_buffer(buffer: &QuatTreeBuffer, theme: &Theme, backend: &mut impl RendererBackend) {
    backend.begin_frame();
    
    for (byte_index, byte) in buffer.bytes.iter().enumerate() {
        for sub_idx in 0..4 {
            let node_index = byte_index * 4 + sub_idx;
            if let Some(quat) = byte.get(sub_idx) {
                backend.draw_node(node_index, quat.to_bits(), theme);
            }
        }
    }
    
    backend.end_frame();
}