pub mod quat;
pub mod state;
pub mod widget;
pub mod container;
pub mod event;
pub mod layout;
pub mod tree_buffer;
pub mod ffi;

pub use quat::{Quat, QuatByte};
pub use state::UiState;
pub use widget::{BasicWidget, Widget};
pub use container::Container;
pub use event::UiEvent;
pub use layout::LayoutMode;
pub use tree_buffer::QuatTreeBuffer;

#[cfg(test)]
mod tests {
    use crate::{BasicWidget, Container, LayoutMode, Quat, QuatByte, QuatTreeBuffer, UiEvent, UiState, Widget};
    use crate::ffi::quat_byte_set_safe;

    // ==========================================
    // 1. NAJLAKŠI TESTOVI (Osnovne jedinice i bitovi)
    // ==========================================

    #[test]
    fn test_01_quat_basic_bits() {
        assert_eq!(Quat::from_bits(0b00), Quat::V00);
        assert_eq!(Quat::from_bits(0b01), Quat::V01);
        assert_eq!(Quat::from_bits(0b10), Quat::V10);
        assert_eq!(Quat::from_bits(0b11), Quat::V11);
        // Test masking behavior for higher bits
        assert_eq!(Quat::from_bits(0b1111), Quat::V11);
    }

    #[test]
    fn test_02_quat_byte_default() {
        let q_byte = QuatByte::new();
        assert_eq!(q_byte.0, 0);
        for i in 0..4 {
            assert_eq!(q_byte.get(i), Some(Quat::V00));
        }
    }

    // ==========================================
    // 2. LAKI DO SREDNJI TESTOVI (Upravljanje bajtom i stanjima)
    // ==========================================

    #[test]
    fn test_03_quat_byte_single_set_get() {
        let mut q_byte = QuatByte::new();
        assert!(q_byte.set(2, Quat::V10).is_ok());
        assert_eq!(q_byte.get(2), Some(Quat::V10));
        // Other indices should remain V00
        assert_eq!(q_byte.get(0), Some(Quat::V00));
    }

    #[test]
    fn test_04_quat_byte_out_of_bounds() {
        let mut q_byte = QuatByte::new();
        assert!(q_byte.set(4, Quat::V11).is_err());
        assert!(q_byte.set(100, Quat::V01).is_err());
        assert_eq!(q_byte.get(4), None);
    }

    #[test]
    fn test_05_quat_byte_full_packing() {
        let mut q_byte = QuatByte::new();
        q_byte.set(0, Quat::V00).unwrap(); // 00
        q_byte.set(1, Quat::V01).unwrap(); // 01
        q_byte.set(2, Quat::V10).unwrap(); // 10
        q_byte.set(3, Quat::V11).unwrap(); // 11
        // Binary representation: 00 01 10 11 -> 00011011 = 27 in decimal
        assert_eq!(q_byte.0, 0b00011011);
        assert_eq!(q_byte.get(0), Some(Quat::V00));
        assert_eq!(q_byte.get(1), Some(Quat::V01));
        assert_eq!(q_byte.get(2), Some(Quat::V10));
        assert_eq!(q_byte.get(3), Some(Quat::V11));
    }

    #[test]
    fn test_06_ui_state_defaults() {
        let default_state = UiState::default();
        assert_eq!(default_state.quat, Quat::V01); // Normal state
        assert!(!default_state.is_disabled());
        assert!(!default_state.is_active());

        let active_state = UiState::new(Quat::V11);
        assert!(active_state.is_active());
    }

    // ==========================================
    // 3. SREDNJI TESTOVI (Događaji, Widgeti i Layout)
    // ==========================================

    #[test]
    fn test_07_ui_event_mapping() {
        assert_eq!(UiEvent::Disable.target_quat(), Quat::V00);
        assert_eq!(UiEvent::Enable.target_quat(), Quat::V01);
        assert_eq!(UiEvent::FocusGained.target_quat(), Quat::V10);
        assert_eq!(UiEvent::Press.target_quat(), Quat::V11);
    }

    #[test]
    fn test_08_basic_widget_state_update() {
        let mut widget = BasicWidget::new("TestButton");
        assert_eq!(widget.state().quat, Quat::V01);

        widget.set_state(UiState::new(Quat::V10));
        assert_eq!(widget.state().quat, Quat::V10);
    }

    #[test]
    fn test_09_layout_mode_flags() {
        let layout_normal = LayoutMode::default();
        assert!(!layout_normal.is_collapsed());
        assert!(!layout_normal.is_expanded());

        let layout_collapsed = LayoutMode::new(Quat::V00, Quat::V00);
        assert!(layout_collapsed.is_collapsed());

        let layout_expanded = LayoutMode::new(Quat::V11, Quat::V01);
        assert!(layout_expanded.is_expanded());
    }

    #[test]
    fn test_10_container_widget_hierarchy() {
        let mut container = Container::new("RootPanel");
        assert_eq!(container.child_count(), 0);

        container.add_child(BasicWidget::new("Child1"));
        container.add_child(BasicWidget::new("Child2"));
        assert_eq!(container.child_count(), 2);
    }

    // ==========================================
    // 4. NAPREDNI I STRES TESTOVI (Tree Buffer pod opterećenjem)
    // ==========================================

    #[test]
    fn test_11_tree_buffer_medium_capacity() {
        let mut buffer = QuatTreeBuffer::new(100);
        assert!(buffer.memory_usage_bytes() > 0);

        // Set boundaries
        assert!(buffer.set_node_state(0, Quat::V11).is_ok());
        assert!(buffer.set_node_state(99, Quat::V10).is_ok());
        assert!(buffer.set_node_state(100, Quat::V01).is_err()); // Out of bounds

        assert_eq!(buffer.get_node_state(0), Some(Quat::V11));
        assert_eq!(buffer.get_node_state(99), Some(Quat::V10));
        assert_eq!(buffer.get_node_state(100), None);
    }

    #[test]
    fn test_12_stress_test_tree_buffer_heavy_churn() {
        // STRESS TEST: Alociramo 10,000 čvorova i vršimo hiljade nasumičnih upisa i čitanja
        const NODE_COUNT: usize = 10_000;
        let mut buffer = QuatTreeBuffer::new(NODE_COUNT);

        // Iteration 1: Set all nodes based on a mathematical pattern
        for i in 0..NODE_COUNT {
            let quat = match i % 4 {
                0 => Quat::V00,
                1 => Quat::V01,
                2 => Quat::V10,
                _ => Quat::V11,
            };
            assert!(buffer.set_node_state(i, quat).is_ok());
        }

        // Iteration 2: Verify all 10,000 nodes under stress loops
        for pass in 0..5 {
            for i in 0..NODE_COUNT {
                let expected = match (i + pass) % 4 {
                    0 => Quat::V00,
                    1 => Quat::V01,
                    2 => Quat::V10,
                    _ => Quat::V11,
                };
                
                // Rotiramo stanje u svakom prolazu da testiramo intenzivno pisanje
                let current_quat = match pass % 4 {
                    0 => Quat::V00,
                    1 => Quat::V01,
                    2 => Quat::V10,
                    _ => Quat::V11,
                };
                
                assert!(buffer.set_node_state(i, current_quat).is_ok());
                assert_eq!(buffer.get_node_state(i), Some(current_quat));
            }
        }

        // Proveravamo da memorijska potrošnja ostaje stabilna i optimizovana (10000 / 4 = 2500 bajtova)
        assert_eq!(buffer.memory_usage_bytes(), (NODE_COUNT + 3) / 4 * std::mem::size_of::<QuatByte>());
    }
    #[test]
fn test_13_ffi_null_pointer_safety() {
    // Testira da FFI bezbedno upravlja null pokazivačima bez panic-a
    let error_code = unsafe {
        quat_byte_set_safe(std::ptr::null_mut(), 0, 0b01)
    };
    // Pretpostavljajući da je QuatUiError::NullPointer vrednost 1
    assert_eq!(error_code as u32, 1);
}

#[test]
fn test_14_layout_resolution() {
    let mode = LayoutMode::default();
    let rect = mode.resolve_layout(800.0, 600.0);
    // Proverava da li default layout ispravno postavlja dimenzije
    assert!(rect.width > 0.0);
    assert!(rect.height > 0.0);
}

}