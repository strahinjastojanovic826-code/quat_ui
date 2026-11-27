use crate::tree_buffer::QuatTreeBuffer;
use crate::theme::Theme;
use crate::layout::LayoutRect;
use crate::quat::Quat;
use crate::theme::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FrameMetrics {
    pub commands_emitted: usize,
    pub dirty_nodes_processed: usize,
    pub memory_bytes: usize,
}

pub trait DiagnosticsSink {
    fn log(&mut self, level: LogLevel, target: &str, message: &str);
    fn record_frame_metrics(&mut self, _metrics: FrameMetrics) {}
}

#[derive(Debug, Default)]
pub struct NullDiagnostics;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderCommand {
    pub node_index: usize,
    pub state: Quat,
    pub rect: LayoutRect,
    pub color: Color,
}

pub trait RendererBackend {
    fn begin_frame(&mut self);
    fn push_clip(&mut self, _rect: LayoutRect) {}
    fn pop_clip(&mut self) {}
    fn draw_command(&mut self, cmd: RenderCommand);
    fn end_frame(&mut self);
}

pub fn render_tree_buffer_incremental(
    buffer: &QuatTreeBuffer,
    dirty_flags: &[bool],
    theme: &Theme,
    viewport_width: f32,
    viewport_height: f32,
    clear_after_render: &mut bool,
    mutable_dirty: &mut [bool],
    backend: &mut impl RendererBackend,
) {
    backend.begin_frame();
    let rect = LayoutRect {
        x: 0.0,
        y: 0.0,
        width: viewport_width,
        height: viewport_height,
    };

    let limit = buffer.capacity_nodes.min(dirty_flags.len());
    for node_index in 0..limit {
        if dirty_flags[node_index] {
            if let Some(quat) = buffer.get_node_state(node_index) {
                let color = theme.resolve_color(quat);
                backend.draw_command(RenderCommand {
                    node_index,
                    state: quat,
                    rect,
                    color,
                });
            }
        }
    }

    if *clear_after_render {
        let clear_len = mutable_dirty.len().min(limit);
        for flag in &mut mutable_dirty[..clear_len] {
            *flag = false;
        }
        *clear_after_render = false;
    }

    backend.end_frame();
}

impl DiagnosticsSink for NullDiagnostics {
    fn log(&mut self, _level: LogLevel, _target: &str, _message: &str) {}
}

pub fn render_with_diagnostics(
    buffer: &QuatTreeBuffer,
    dirty_flags: &[bool],
    theme: &Theme,
    viewport_width: f32,
    viewport_height: f32,
    backend: &mut impl RendererBackend,
    diagnostics: &mut impl DiagnosticsSink,
) -> FrameMetrics {
    diagnostics.log(
        LogLevel::Trace,
        "quat_ui::renderer",
        &buffer.debug_summary(),
    );

    let mut clear_after = false;
    let mut dummy_dirty = vec![false; buffer.capacity_nodes];
    let mut dirty_count = 0usize;
    let mut cmd_count = 0usize;

    struct CountingBackend<'a, B: RendererBackend> {
        inner: &'a mut B,
        cmd_count: &'a mut usize,
    }

    impl<'a, B: RendererBackend> RendererBackend for CountingBackend<'a, B> {
        fn begin_frame(&mut self) {
            self.inner.begin_frame();
        }
        fn push_clip(&mut self, rect: LayoutRect) {
            self.inner.push_clip(rect);
        }
        fn pop_clip(&mut self) {
            self.inner.pop_clip();
        }
        fn draw_command(&mut self, cmd: RenderCommand) {
            *self.cmd_count += 1;
            self.inner.draw_command(cmd);
        }
        fn end_frame(&mut self) {
            self.inner.end_frame();
        }
    }

    for &is_dirty in dirty_flags.iter().take(buffer.capacity_nodes) {
        if is_dirty {
            dirty_count += 1;
        }
    }

    let mut counting = CountingBackend {
        inner: backend,
        cmd_count: &mut cmd_count,
    };

    render_tree_buffer_incremental(
        buffer,
        dirty_flags,
        theme,
        viewport_width,
        viewport_height,
        &mut clear_after,
        &mut dummy_dirty,
        &mut counting,
    );

    let metrics = FrameMetrics {
        commands_emitted: cmd_count,
        dirty_nodes_processed: dirty_count,
        memory_bytes: buffer.memory_usage_bytes(),
    };

    diagnostics.record_frame_metrics(metrics);
    metrics
}

pub fn render_tree_buffer(
    buffer: &QuatTreeBuffer,
    theme: &Theme,
    viewport_width: f32,
    viewport_height: f32,
    backend: &mut impl RendererBackend,
) {
    backend.begin_frame();
    let default_rect = LayoutRect {
        x: 0.0,
        y: 0.0,
        width: viewport_width,
        height: viewport_height,
    };

    for (byte_index, byte) in buffer.bytes.iter().enumerate() {
        for sub_idx in 0..4 {
            let node_index = byte_index * 4 + sub_idx;
            if node_index >= buffer.capacity_nodes {
                break;
            }
            if let Some(quat) = byte.get(sub_idx) {
                let color = theme.resolve_color(quat);
                backend.draw_command(RenderCommand {
                    node_index,
                    state: quat,
                    rect: default_rect,
                    color,
                });
            }
        }
    }

    backend.end_frame();
}