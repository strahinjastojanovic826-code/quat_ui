use std::ffi::CStr;
use std::os::raw::c_char;
use crate::{BasicWidget, Container, Quat, QuatByte, QuatTreeBuffer, UiEvent, UiState, Widget};

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