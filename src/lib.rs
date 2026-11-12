pub mod quat;
pub mod state;
pub mod widget;
pub mod container;
pub mod event;
pub mod layout;
pub mod tree_buffer;
pub mod ffi;
pub mod theme;
pub mod renderer;

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
    use std::time::Instant;
    use crate::ffi::QuatUiError;
    use crate::layout::LayoutRect;

    // ==========================================
    // 1. NAJLAKŠI TESTOVI (Osnovne jedinice i bitovi)
    // ==========================================

    #[test]
    fn test_01_quat_basic_bits() {
        let start = Instant::now();
        
        assert_eq!(Quat::from_bits(0b00), Quat::V00);
        assert_eq!(Quat::from_bits(0b01), Quat::V01);
        assert_eq!(Quat::from_bits(0b10), Quat::V10);
        assert_eq!(Quat::from_bits(0b11), Quat::V11);
        // Test masking behavior for higher bits
        assert_eq!(Quat::from_bits(0b1111), Quat::V11);

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_01_quat_basic_bits took: {:?}", duration);
    }

    #[test]
    fn test_02_quat_byte_default() {
        let start = Instant::now();
        
        let q_byte: QuatByte = QuatByte::new();
        assert_eq!(q_byte.0, 0);
        for i in 0..4 {
            assert_eq!(q_byte.get(i), Some(Quat::V00));
        }

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_02_quat_byte_default took: {:?}", duration);
    }

    // ==========================================
    // 2. LAKI DO SREDNJI TESTOVI (Upravljanje bajtom i stanjima)
    // ==========================================

    #[test]
    fn test_03_quat_byte_single_set_get() {
        let start = Instant::now();
        
        let mut q_byte: QuatByte = QuatByte::new();
        assert!(q_byte.set(2, Quat::V10).is_ok());
        assert_eq!(q_byte.get(2), Some(Quat::V10));
        // Other indices should remain V00
        assert_eq!(q_byte.get(0), Some(Quat::V00));

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_03_quat_byte_single_set_get took: {:?}", duration);
    }

    #[test]
    fn test_04_quat_byte_out_of_bounds() {
        let start = Instant::now();
        
        let mut q_byte: QuatByte = QuatByte::new();
        assert!(q_byte.set(4, Quat::V11).is_err());
        assert_eq!(q_byte.get(4), None);

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_04_quat_byte_out_of_bounds took: {:?}", duration);
    }

    #[test]
    fn test_05_quat_byte_full_packing() {
        let start = Instant::now();
        
        let mut q_byte: QuatByte = QuatByte::new();
        q_byte.set(0, Quat::V00).unwrap(); // 00
        q_byte.set(1, Quat::V01).unwrap(); // 01
        q_byte.set(2, Quat::V10).unwrap(); // 10
        q_byte.set(3, Quat::V11).unwrap(); // 11
        // Binary representation: 00 01 10 11 -> 00011011 = 27 in decimal
        assert_eq!(q_byte.0, 0b00011011);
        assert_eq!(q_byte.get(0), Some(Quat::V00));
        assert_eq!(q_byte.get(2), Some(Quat::V10));
        assert_eq!(q_byte.get(3), Some(Quat::V11));

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_05_quat_byte_full_packing took: {:?}", duration);
    }

    #[test]
    fn test_06_ui_state_defaults() {
        let start = Instant::now();
        
        let default_state: UiState = UiState::default();
        assert_eq!(default_state.quat, Quat::V01); // Normal state
        assert!(!default_state.is_disabled());
        assert!(!default_state.is_active());

        let active_state: UiState = UiState::new(Quat::V11);
        assert!(active_state.is_active());

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_06_ui_state_defaults took: {:?}", duration);
    }

    // ==========================================
    // 3. SREDNJI TESTOVI (Događaji, Widgeti i Layout)
    // ==========================================

    #[test]
    fn test_07_ui_event_mapping() {
        let start = Instant::now();
        
        assert_eq!(UiEvent::Disable.target_quat(), Quat::V00);
        assert_eq!(UiEvent::Enable.target_quat(), Quat::V01);
        assert_eq!(UiEvent::FocusGained.target_quat(), Quat::V10);
        assert_eq!(UiEvent::Press.target_quat(), Quat::V11);

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_07_ui_event_mapping took: {:?}", duration);
    }

    #[test]
    fn test_08_basic_widget_state_update() {
        let start = Instant::now();
        
        let mut widget: BasicWidget = BasicWidget::new("TestButton");
        assert_eq!(widget.state().quat, Quat::V01);

        widget.set_state(UiState::new(Quat::V10));
        assert_eq!(widget.state().quat, Quat::V10);

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_08_basic_widget_state_update took: {:?}", duration);
    }

    #[test]
    fn test_09_layout_mode_flags() {
        let start = Instant::now();
        
        let layout_normal: LayoutMode = LayoutMode::default();
        assert!(!layout_normal.is_collapsed());
        assert!(!layout_normal.is_expanded());

        let layout_collapsed: LayoutMode = LayoutMode::new( Quat::V00, Quat::V00);
        assert!(layout_collapsed.is_collapsed());

        let layout_expanded: LayoutMode = LayoutMode::new( Quat::V11, Quat::V01);
        assert!(layout_expanded.is_expanded());

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_09_layout_mode_flags took: {:?}", duration);
    }

    #[test]
    fn test_10_container_widget_hierarchy() {
        let start = Instant::now();
        
        let mut container: Container = Container::new("RootPanel");
        assert_eq!(container.child_count(), 0);

        container.add_child(BasicWidget::new("Child1"));
        container.add_child( BasicWidget::new("Child2"));
        assert_eq!(container.child_count(), 2);

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_10_container_widget_hierarchy took: {:?}", duration);
    }

    // ==========================================
    // 4. NAPREDNI I STRES TESTOVI (Tree Buffer pod opterećenjem)
    // ==========================================

    #[test]
    fn test_11_tree_buffer_medium_capacity() {
        let start = Instant::now();
        
        let mut buffer: QuatTreeBuffer = QuatTreeBuffer::new(100);
        assert!(buffer.memory_usage_bytes() > 0);

        // Set boundaries
        assert!(buffer.set_node_state(0, Quat::V11).is_ok());
        assert!(buffer.set_node_state(99, Quat::V10).is_ok());
        assert!(buffer.set_node_state(100, Quat::V01).is_err()); // Out of bounds

        assert_eq!(buffer.get_node_state(0), Some(Quat::V11));
        assert_eq!(buffer.get_node_state(99), Some(Quat::V10));
        assert_eq!(buffer.get_node_state(100), None);

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_11_tree_buffer_medium_capacity took: {:?}", duration);
    }

    #[test]
    fn test_12_stress_test_tree_buffer_heavy_churn() {
        let start = Instant::now();
        
        // STRESS TEST: Alociramo 10,000 čvorova i vršimo hiljade nasumičnih upisa i čitanja
        const NODE_COUNT: usize = 10_000;
        let mut buffer: QuatTreeBuffer = QuatTreeBuffer::new(NODE_COUNT);

        // Iteration 1: Set all nodes based on a mathematical pattern
        for i in 0..NODE_COUNT {
            let quat: Quat = match i % 4 {
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
                let current_quat: Quat = match (i + pass) % 4 {
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

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_12_stress_test_tree_buffer_heavy_churn took: {:?}", duration);
    }

    #[test]
    fn test_13_ffi_null_pointer_safety() {
        let start = Instant::now();
        
        // Testira da FFI bezbedno upravlja null pokazivačima bez panic-a
        let error_code: QuatUiError = unsafe {
            quat_byte_set_safe(std::ptr::null_mut(), 0, 0b01)
        };
        // Pretpostavljajući da je QuatUiError::NullPointer vrednost 1
        assert_eq!(error_code as u32, 1);

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_13_ffi_null_pointer_safety took: {:?}", duration);
    }

    #[test]
    fn test_14_layout_resolution() {
        let start = Instant::now();
        
        let mode: LayoutMode = LayoutMode::default();
        let rect: LayoutRect = mode.resolve_layout(800.0, 600.0);
        // Proverava da li default layout ispravno postavlja dimenzije
        assert!(rect.width > 0.0);
        assert!(rect.height > 0.0);

        let duration = start.elapsed();
        println!("\n[BENCHMARK] test_14_layout_resolution took: {:?}", duration);
    }

#[test]
    fn test_15_theme_resolution_benchmark() {
        use crate::theme::Theme;
        use crate::quat::Quat;
        use std::time::Instant;

        let start = Instant::now();
        let theme = Theme::default_dark();

        // Benchmark work: resolving theme colors across states multiple times
        for _ in 0..10_000 {
            let _ = theme.resolve_color(Quat::V00);
            let _ = theme.resolve_color(Quat::V01);
            let _ = theme.resolve_color(Quat::V10);
            let _ = theme.resolve_color(Quat::V11);
        }
        let duration = start.elapsed();

        assert_eq!(theme.resolve_color(Quat::V00), theme.background_disabled);
        println!("\n[BENCHMARK] test_15_theme_resolution_benchmark took: {:?}", duration);
    }

    #[test]
    fn test_16_container_scrolling_benchmark() {
        use crate::container::Container;
        use std::time::Instant;

        let start = Instant::now();
        let mut container = Container::new("BenchContainer");

        // Benchmark work: manipulating scroll states iteratively
        for i in 0..10_000 {
            container.scroll_y = i as f32;
            container.max_scroll_y = (i * 2) as f32;
        }
        let duration = start.elapsed();

        assert_eq!(container.scroll_y, 9999.0);
        println!("\n[BENCHMARK] test_16_container_scrolling_benchmark took: {:?}", duration);
    }

    #[test]
    fn test_17_renderer_backend_benchmark() {
        use crate::tree_buffer::QuatTreeBuffer;
        use crate::theme::Theme;
        use crate::renderer::{RendererBackend, render_tree_buffer};
        use std::time::Instant;

        struct LightMockBackend {
            count: usize,
        }
        impl RendererBackend for LightMockBackend {
            fn begin_frame(&mut self) { self.count = 0; }
            fn draw_node(&mut self, _idx: usize, _bits: u8, _theme: &Theme) { self.count += 1; }
            fn end_frame(&mut self) {}
        }

        let buffer = QuatTreeBuffer::new(1000);
        let theme = Theme::default_dark();
        let mut backend = LightMockBackend { count: 0 };

        let start = Instant::now();
        // Benchmark work: render buffer once inside this test
        render_tree_buffer(&buffer, &theme, &mut backend);
        let duration = start.elapsed();

        assert_eq!(backend.count, 1000);
        println!("\n[BENCHMARK] test_17_renderer_backend_benchmark took: {:?}", duration);
    }

}