# QuatUI

**QuatUI** is a high-performance, modular UI library written in Rust that leverages **quaternary logic** (4 states packed into 2 bits: `00`, `01`, `10`, `11`). It features a Data-Oriented Design (DOD) tree buffer for handling thousands of UI nodes with minimal memory footprint, a flexible layout engine, an event translation system, and robust FFI bindings for cross-language integration (C#, Python, C++, etc.).

---

## FEATURES

* **Quaternary Logic (2-bit States):** Uses `Quat` enums based on 2-bit values (`V00`, `V01`, `V10`, `V11`) for compact state representation.
* **Byte Packing (`QuatByte`):** Packs up to 4 quaternary nodes into a single `u8` byte for minimal memory footprint.
* **High-Performance Memory Buffer (`QuatTreeBuffer`):** A data-oriented structure designed to store and manage the state of thousands of UI nodes efficiently.
* **Widget Hierarchy & Containers:** Flexible container (`Container`) and widget (`BasicWidget`, `Widget` trait) systems for building nested UI structures.
* **Layout Engine (`LayoutMode`):** Encodes layout behavior (collapsed, relative, absolute, expanded) directly through quaternary states.
* **Event Management (`UiEvent`):** Robust mapping of high-level user interactions and system triggers into quaternary target states.
* **Universal FFI Layer (C / C# / Python Interop):** Stable external interface with memory management and interoperability functions for other programming languages[cite: 9, 10, 11, 12].
* **Comprehensive Testing:** Thorough suite of unit tests and heavy stress tests covering thousands of nodes in real-time.
* **Theme Engine (`Theme` & `Color`):** Automatic mapping of quaternary states into visual themes and colors.
* **Container Scrolling (`scroll_y` & `max_scroll_y`):** Built-in vertical scroll states and offset management for container widgets.
* **Renderer Backend (`RendererBackend`):** Abstract rendering bridge designed to map quaternary UI buffers onto graphical targets.
* **Arena & Index-Based Hierarchy (`NodeArena`, `NodeId`, `NodeMeta`):** Index-addressed node hierarchy (`parent`, `first_child`, `next_sibling`) paired with packed quaternary state buffers.
* **Quaternary State Machine & Event Dispatch (`UiEvent::next_state`, `dispatch_event`):** Deterministic state transitions respecting `Disabled` (`00`) guards, `Press`/`Release` flow, and child state propagation.
* **Batch Operations & State Diffing (`QuatTreeBuffer`):** Range updates (`set_range`, `fill`, `clear_all`), state queries (`indices_by_state`, `count_in_range`), and change detection (`diff_indices`).
* **Dirty Tracking & Incremental Rendering:** Tracks modified node indices (`set_node_state_tracked`, `mark_dirty`) to support incremental rendering (`render_tree_buffer_incremental`).
* **Snapshots, Diagnostics & Frame Metrics:** Byte-level snapshots (`QuatSnapshot`), state distribution stats (`QuatStateStats`), pluggable logging (`DiagnosticsSink`), and per-frame metrics (`FrameMetrics`).
* **Expanded Layout & Arena FFI:** Supports explicit relative/absolute sizing (`resolve_detailed`), rich `RenderCommand` output (`rect` and `color`), and C ABI lifecycle functions (`quat_arena_*`).

---

## ⚙️ Core Concepts

The library encodes UI states into 2-bit quaternary values (`Quat`):
* **`00`**: Disabled / Collapsed
* **`01`**: Normal / Idle / Relative Flow
* **`10`**: Hovered / Focused / Absolute Flow
* **`11`**: Active / Pressed / Expanded Flow

Using `QuatByte`, **4 quaternary states are packed into a single standard byte (`u8`)**, ensuring ultra-high cache locality and minimal memory usage.

---

## Benchmark

| # | Test Name | Category | Measured Time | Status |
| :--- | :--- | :--- | :--- | :--- |
| 01 | `test_01_quat_basic_bits` | Quaternary Primitives | `5.6µs` | `ok` |
| 02 | `test_02_quat_byte_default` | Byte Packing | `10.1µs` | `ok` |
| 03 | `test_03_quat_byte_single_set_get` | Byte Packing | `300ns` | `ok` |
| 04 | `test_04_quat_byte_out_of_bounds` | Bounds & Validation | `200ns` | `ok` |
| 05 | `test_05_quat_byte_full_packing` | Byte Packing | `600ns` | `ok` |
| 06 | `test_06_ui_state_defaults` | State Defaults | `100ns` | `ok` |
| 07 | `test_07_ui_event_mapping` | Event Mapping | `200ns` | `ok` |
| 08 | `test_08_basic_widget_state_update` | Widget State | `23.4µs` | `ok` |
| 09 | `test_09_layout_mode_flags` | Layout Engine | `200ns` | `ok` |
| 10 | `test_10_container_widget_hierarchy` | Widget Hierarchy | `14.3µs` | `ok` |
| 11 | `test_11_tree_buffer_medium_capacity` | Tree Buffer | `2.1µs` | `ok` |
| 12 | `test_12_stress_test_tree_buffer_heavy_churn` | Stress & Memory | `2.5968ms` | `ok` |
| 13 | `test_13_ffi_null_pointer_safety` | FFI Safety | `300ns` | `ok` |
| 14 | `test_14_layout_resolution` | Layout Engine | `200ns` | `ok` |
| 15 | `test_15_theme_resolution_benchmark` | Theme Engine | `160.4µs` | `ok` |
| 16 | `test_16_container_scrolling_benchmark` | Scrolling | `57.9µs` | `ok` |
| 17 | `test_17_renderer_backend_benchmark` | Renderer Backend | `29.7µs` | `ok` |
| 18 | `test_18_ui_event_transitions_benchmark` | State Machine | `204.5µs` | `ok` |
| 19 | `test_19_container_dispatch_event_benchmark` | Event Dispatch | `211.1µs` | `ok` |
| 20 | `test_20_layout_detailed_resolution_benchmark` | Layout Engine | `127.5µs` | `ok` |
| 21 | `test_21_tree_buffer_range_query_and_diff_benchmark` | Buffer Query & Diff | `59.1524ms` | `ok` |
| 22 | `test_22_dirty_tracking_snapshot_and_diagnostics_benchmark` | Dirty, Snapshot & Diag | `3.4603ms` | `ok` |
| 23 | `test_23_node_arena_and_ffi_lifecycle_benchmark` | Arena & FFI Lifecycle | `902.4µs` | `ok` |

---

## Licensing

This project is dual-licensed:

* **Open Source (AGPLv3):** Free to use for open-source projects. If you use or modify this library in your software or network service, your source code must also be open-source under AGPLv3.
* **Commercial License:** Required if you want to use `quat_ui` in closed-source or proprietary commercial software without AGPLv3 restrictions.

For commercial licensing, contact: **strahinjastojanovic826@gmail.com**

---

## 📦 Installation (`Cargo.toml`)

Add the library to your Rust project configuration with `cdylib` support for FFI:

```toml
[dependencies]
quat_ui = "0.6.0"
```

---

## 🚀 Quick Start (Rust)

```rust
use quat_ui::{BasicWidget, Container, UiEvent, UiState, Widget};

fn main() {
    // 1. Create a root container panel
    let mut root_panel = Container::new("MainPanel");

    // 2. Create child widgets
    let btn1 = BasicWidget::new("LoginButton");
    let btn2 = BasicWidget::new("CancelButton");

    // 3. Build widget hierarchy
    root_panel.add_child(btn1);
    root_panel.add_child(btn2);

    println!("--- Initial Render ---");
    root_panel.render();

    // 4. Dispatch a focus event through the container state machine
    let changed = root_panel.dispatch_event(&UiEvent::FocusGained);
    let next_q = UiEvent::FocusGained.next_state(root_panel.state().quat);
    root_panel.set_state(UiState::new(next_q));

    println!("\n--- Render After Focus Event (changed: {}) ---", changed);
    root_panel.render();
}
```

---

## Themes and Scrolling

```rust
use quat_ui::{Container, Quat, Theme};

fn main() {
    // 1. Initialize container with scroll values
    let mut root_panel = Container::new("MainPanel");
    root_panel.scroll_y = 50.0;
    root_panel.max_scroll_y = 300.0;

    // 2. Setup theme mapping based on quaternary states
    let theme = Theme::default_dark();
    let active_color = theme.resolve_color(Quat::V11); // Maps V11 (Active/Pressed) to theme color

    println!(
        "Container scroll Y: {}, Max scroll Y: {}",
        root_panel.scroll_y, root_panel.max_scroll_y
    );
    println!(
        "Resolved active theme color - R: {}, G: {}, B: {}",
        active_color.r, active_color.g, active_color.b
    );
}
```

---

## Buffer Operations, Snapshots & Dirty Tracking

```rust
use quat_ui::{Quat, QuatTreeBuffer};

fn main() {
    let mut buffer = QuatTreeBuffer::new(16);
    let mut dirty = vec![false; 16];

    buffer.set_range(0, 4, Quat::V10).unwrap();
    let changed = buffer.set_node_state_tracked(2, Quat::V11, &mut dirty).unwrap();

    let snapshot = buffer.snapshot();
    println!("State changed: {}, dirty[2]: {}", changed, dirty[2]);
    println!("{}", buffer.debug_summary());

    buffer.clear_all();
    buffer.restore_snapshot(&snapshot).unwrap();
}
```

---

## 🌐 FFI Integration (C# / Python)

Thanks to the built-in C-compatible FFI layer (`src/ffi.rs`), you can compile `quat_ui` into a dynamic library (`.dll`, `.so`, or `.dylib`) and use it from other languages.

### C# (.NET) Example:

```csharp
using System;
using System.Runtime.InteropServices;

class Program
{
    [DllImport("quat_ui", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr quat_arena_create(UIntPtr capacity_nodes);

    [DllImport("quat_ui", CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
    private static extern int quat_arena_add_basic_widget(
        IntPtr handle,
        string id_ptr,
        out UIntPtr out_node_id
    );

    [DllImport("quat_ui", CallingConvention = CallingConvention.Cdecl)]
    private static extern int quat_arena_set_quat(
        IntPtr handle,
        UIntPtr node_id,
        byte quat_val
    );

    [DllImport("quat_ui", CallingConvention = CallingConvention.Cdecl)]
    private static extern int quat_arena_destroy(IntPtr handle);

    static void Main()
    {
        IntPtr arena = quat_arena_create((UIntPtr)64);
        if (arena == IntPtr.Zero) return;

        quat_arena_add_basic_widget(arena, "SubmitButton", out UIntPtr nodeId);
        quat_arena_set_quat(arena, nodeId, 0b11); // V11 (Active / Pressed)
        quat_arena_destroy(arena);
    }
}
```

---

### Python Example:

```python
import ctypes
import sys

if sys.platform.startswith("win"):
    lib_path = "./target/release/quat_ui.dll"
elif sys.platform.startswith("darwin"):
    lib_path = "./target/release/libquat_ui.dylib"
else:
    lib_path = "./target/release/libquat_ui.so"

lib = ctypes.CDLL(lib_path)

lib.quat_arena_create.argtypes = [ctypes.c_size_t]
lib.quat_arena_create.restype = ctypes.c_void_p

lib.quat_arena_add_basic_widget.argtypes = [
    ctypes.c_void_p,
    ctypes.c_char_p,
    ctypes.POINTER(ctypes.c_size_t),
]
lib.quat_arena_add_basic_widget.restype = ctypes.c_int

lib.quat_arena_set_quat.argtypes = [
    ctypes.c_void_p,
    ctypes.c_size_t,
    ctypes.c_uint8,
]
lib.quat_arena_set_quat.restype = ctypes.c_int

lib.quat_arena_destroy.argtypes = [ctypes.c_void_p]
lib.quat_arena_destroy.restype = ctypes.c_int

arena = lib.quat_arena_create(64)
if arena:
    node_id = ctypes.c_size_t(0)
    lib.quat_arena_add_basic_widget(arena, b"PythonButton", ctypes.byref(node_id))
    lib.quat_arena_set_quat(arena, node_id.value, 0b10)  # V10 (Focused / Hover)
    lib.quat_arena_destroy(arena)
```

---

## 🧪 Running Tests

QuatUI includes a comprehensive suite of unit tests ranging from basic bit manipulation to heavy stress tests with 10,000+ nodes in the tree buffer.

Run tests using Cargo:
```bash
cargo test
```