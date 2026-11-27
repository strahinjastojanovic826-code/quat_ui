//! C-compatible Foreign Function Interface (FFI) layer for `quat_ui`.
//!
//! # Safety Invariants
//! All exported `extern "C"` functions and internal `unsafe` blocks in this module
//! cross the FFI boundary and operate on raw pointers (`*const` / `*mut`), C strings,
//! and heap-allocated handles (`Box::into_raw` / `Box::from_raw`). Callers across
//! FFI (such as C, C++, C#, or Python) must uphold the following invariants:
//!
//! - **Non-null & lifetime validity:** Any non-null pointer passed into these functions
//!   must point to a valid, properly aligned, and initialized instance of its expected
//!   type for the duration of the call, and must not alias mutable references concurrently.
//! - **Ownership & destruction:** Handles allocated via `*_new` or `quat_arena_create`
//!   must be released at most once using their matching `*_free` or `quat_arena_destroy`
//!   function. Passing an already-freed or foreign pointer is undefined behavior.
//! - **C strings & callbacks:** `id_ptr` must point to a valid, null-terminated UTF-8
//!   string for the call duration. In `quat_arena_emit_commands`, `sink` and its
//!   `on_command` callback must remain valid and safe to invoke for the duration of
//!   the render pass.
//! - **Panic & state bounds:** Functions returning `QuatUiError` wrap execution in
//!   `wrap_unwind` to prevent unwinding across the C ABI boundary and validate null
//!   pointers and quaternary/index bounds before dereferencing.

use std::ffi::CStr;
use std::os::raw::c_char;
use crate::{BasicWidget, Container, Quat, QuatByte, QuatTreeBuffer, UiEvent, UiState, Widget};
use crate::container::NodeArena;
use crate::layout::LayoutRect;
use crate::renderer::{render_tree_buffer, RenderCommand, RendererBackend};
use crate::theme::Theme;

#[repr(C)]
pub enum QuatUiError {
    Success = 0,
    NullPointer = 1,
    OutOfBounds = 2,
    InternalPanic = 3,
}

// --- QUAT & QUAT BYTE FFI ---

#[unsafe(no_mangle)]
pub extern "C" fn quat_byte_new() -> *mut QuatByte {
    Box::into_raw(Box::new(QuatByte::new()))
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_byte_free(ptr: *mut QuatByte) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_byte_set(ptr: *mut QuatByte, index: usize, quat_val: u8) -> bool {
    if ptr.is_null() {
        return false;
    }
    unsafe {
        let q_byte = &mut *ptr;
        let quat = Quat::from_bits(quat_val);
        q_byte.set(index, quat).is_ok()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_byte_get(ptr: *const QuatByte, index: usize, out_val: *mut u8) -> bool {
    if ptr.is_null() || out_val.is_null() {
        return false;
    }
    unsafe {
        let q_byte = &*ptr;
        if let Some(quat) = q_byte.get(index) {
            *out_val = quat.to_bits();
            true
        } else {
            false
        }
    }
}

// --- WIDGET FFI ---

#[unsafe(no_mangle)]
pub extern "C" fn quat_widget_new(id_ptr: *const c_char) -> *mut BasicWidget {
    if id_ptr.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(id_ptr) };
    let id_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let widget = BasicWidget::new(id_str);
    Box::into_raw(Box::new(widget))
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_widget_free(ptr: *mut BasicWidget) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_widget_set_state(ptr: *mut BasicWidget, quat_val: u8) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let widget = &mut *ptr;
        let quat = Quat::from_bits(quat_val);
        widget.set_state(UiState::new(quat));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_widget_get_state(ptr: *const BasicWidget) -> u8 {
    if ptr.is_null() {
        return 0;
    }
    unsafe {
        let widget = &*ptr;
        widget.state().quat.to_bits()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_widget_render(ptr: *const BasicWidget) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let widget = &*ptr;
        widget.render();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_widget_handle_event(ptr: *mut BasicWidget, event_type: u8) {
    if ptr.is_null() { return; }
    let event = match event_type {
        0 => UiEvent::Disable,
        1 => UiEvent::Enable,
        2 => UiEvent::FocusGained,
        3 => UiEvent::FocusLost,
        4 => UiEvent::Press,
        _ => UiEvent::Release,
    };
    unsafe {
        let widget = &mut *ptr;
        widget.set_state(UiState::new(event.target_quat()));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_container_new(id_ptr: *const c_char) -> *mut Container {
    if id_ptr.is_null() { return std::ptr::null_mut(); }
    let c_str = unsafe { CStr::from_ptr(id_ptr) };
    let id_str = match c_str.to_str() { Ok(s) => s, Err(_) => return std::ptr::null_mut() };
    Box::into_raw(Box::new(Container::new(id_str)))
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_container_free(ptr: *mut Container) {
    if !ptr.is_null() { unsafe { let _ = Box::from_raw(ptr); } }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_container_add_widget(container_ptr: *mut Container, widget_ptr: *mut BasicWidget) {
    if container_ptr.is_null() || widget_ptr.is_null() { return; }
    unsafe {
        let container = &mut *container_ptr;
        let widget = Box::from_raw(widget_ptr); // Preuzimamo vlasništvo nad widgetom u kontejner
        container.add_child(*widget);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_container_render(ptr: *const Container) {
    if ptr.is_null() { return; }
    unsafe { (&*ptr).render(); }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_tree_buffer_new(max_nodes: usize) -> *mut QuatTreeBuffer {
    Box::into_raw(Box::new(QuatTreeBuffer::new(max_nodes)))
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_tree_buffer_free(ptr: *mut QuatTreeBuffer) {
    if !ptr.is_null() { unsafe { let _ = Box::from_raw(ptr); } }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_tree_buffer_set(ptr: *mut QuatTreeBuffer, index: usize, quat_val: u8) -> bool {
    if ptr.is_null() { return false; }
    unsafe {
        let buffer = &mut *ptr;
        let quat = Quat::from_bits(quat_val);
        buffer.set_node_state(index, quat).is_ok()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_tree_buffer_get(ptr: *const QuatTreeBuffer, index: usize, out_val: *mut u8) -> bool {
    if ptr.is_null() || out_val.is_null() { return false; }
    unsafe {
        let buffer = &*ptr;
        if let Some(quat) = buffer.get_node_state(index) {
            *out_val = quat.to_bits();
            true
        } else {
            false
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_tree_buffer_memory_size(ptr: *const QuatTreeBuffer) -> usize {
    if ptr.is_null() { return 0; }
    unsafe { (&*ptr).memory_usage_bytes() }
}

/// Helper function to safely catch panics across FFI boundaries
fn wrap_unwind<F>(f: F) -> QuatUiError
where
    F: FnOnce() -> Result<(), QuatUiError> + std::panic::UnwindSafe,
{
    match std::panic::catch_unwind(f) {
        Ok(Ok(())) => QuatUiError::Success,
        Ok(Err(e)) => e,
        Err(_) => QuatUiError::InternalPanic,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_ui_get_version(out_major: *mut u32, out_minor: *mut u32) { unsafe {
    if !out_major.is_null() { *out_major = 1; }
    if !out_minor.is_null() { *out_minor = 0; }
}}

// Example of industrial-grade safe FFI function using QuatByte
#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_byte_set_safe(
    ptr: *mut QuatByte,
    index: usize,
    quat_val: u8,
) -> QuatUiError { unsafe {
    if ptr.is_null() {
        return QuatUiError::NullPointer;
    }

    wrap_unwind(|| {
        let q_byte = &mut *ptr;
        let quat = Quat::from_bits(quat_val);
        q_byte.set(index, quat).map_err(|_| QuatUiError::OutOfBounds)
    })
}}

#[repr(C)]
pub struct QuatArenaHandle(pub NodeArena);

#[repr(C)]
pub struct CCommandSink {
    pub user_data: *mut std::ffi::c_void,
    pub on_command: Option<extern "C" fn(*mut std::ffi::c_void, usize, u8, LayoutRect)>,
}

struct FfiBackend<'a> {
    sink: &'a mut CCommandSink,
}

impl<'a> RendererBackend for FfiBackend<'a> {
    fn begin_frame(&mut self) {}

    fn draw_command(&mut self, cmd: RenderCommand) {
        if let Some(cb) = self.sink.on_command {
            cb(self.sink.user_data, cmd.node_index, cmd.state.to_bits(), cmd.rect);
        }
    }

    fn end_frame(&mut self) {}
}

/// Creates a new arena handle for hierarchical nodes and packed quaternary states.
#[unsafe(no_mangle)]
pub extern "C" fn quat_arena_create(initial_capacity: usize) -> *mut QuatArenaHandle {
    let cap = initial_capacity.max(1);
    Box::into_raw(Box::new(QuatArenaHandle(NodeArena::new(cap))))
}

/// Destroys an arena handle created by `quat_arena_create`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_arena_destroy(handle: *mut QuatArenaHandle) -> QuatUiError {
    if handle.is_null() {
        return QuatUiError::NullPointer;
    }
    let addr = handle as usize;
    wrap_unwind(move || {
        unsafe {
            let ptr = addr as *mut QuatArenaHandle;
            let _ = Box::from_raw(ptr);
        }
        Ok(())
    })
}

/// Adds a `BasicWidget` to the arena and returns its node index in `out_node_id`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_arena_add_basic_widget(
    handle: *mut QuatArenaHandle,
    id_ptr: *const c_char,
    out_node_id: *mut usize,
) -> QuatUiError {
    if handle.is_null() || id_ptr.is_null() || out_node_id.is_null() {
        return QuatUiError::NullPointer;
    }
    let handle_addr = handle as usize;
    let id_addr = id_ptr as usize;
    let out_addr = out_node_id as usize;

    wrap_unwind(move || unsafe {
        let c_str = CStr::from_ptr(id_addr as *const c_char);
        let id_str = c_str.to_str().map_err(|_| QuatUiError::OutOfBounds)?;
        let arena = &mut (*(handle_addr as *mut QuatArenaHandle)).0;
        let node_id = arena
            .add_node(None, BasicWidget::new(id_str))
            .map_err(|_| QuatUiError::OutOfBounds)?;
        *(out_addr as *mut usize) = node_id.0;
        Ok(())
    })
}

/// Sets the quaternary state (`0..=3`) of a node in the arena.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_arena_set_quat(
    handle: *mut QuatArenaHandle,
    node_id: usize,
    quat_val: u8,
) -> QuatUiError {
    if handle.is_null() {
        return QuatUiError::NullPointer;
    }
    if quat_val > 0b11 {
        return QuatUiError::OutOfBounds;
    }
    let handle_addr = handle as usize;

    wrap_unwind(move || unsafe {
        let arena = &mut (*(handle_addr as *mut QuatArenaHandle)).0;
        arena
            .set_node_quat(crate::container::NodeId(node_id), Quat::from_bits(quat_val))
            .map_err(|_| QuatUiError::OutOfBounds)
    })
}

/// Emits render commands for the arena's state buffer to a C callback sink.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_arena_emit_commands(
    handle: *const QuatArenaHandle,
    viewport_width: f32,
    viewport_height: f32,
    sink: *mut CCommandSink,
) -> QuatUiError {
    if handle.is_null() || sink.is_null() {
        return QuatUiError::NullPointer;
    }
    let handle_addr = handle as usize;
    let sink_addr = sink as usize;

    wrap_unwind(move || unsafe {
        let arena = &(*(handle_addr as *const QuatArenaHandle)).0;
        let theme = Theme::default_dark();
        let mut backend = FfiBackend {
            sink: &mut *(sink_addr as *mut CCommandSink),
        };
        render_tree_buffer(
            &arena.states,
            &theme,
            viewport_width,
            viewport_height,
            &mut backend,
        );
        Ok(())
    })
}