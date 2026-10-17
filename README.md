# QuatUI

**QuatUI** is a high-performance, modular UI library written in Rust that leverages **quaternary logic** (4 states packed into 2 bits: `00`, `01`, `10`, `11`). It features a Data-Oriented Design (DOD) tree buffer for handling thousands of UI nodes with minimal memory footprint, a flexible layout engine, an event translation system, and robust FFI bindings for cross-language integration (C#, Python, C++, etc.).

---

## ⚙️ Core Concepts

The library encodes UI states into 2-bit quaternary values (`Quat`):
* **`00`**: Disabled / Collapsed
* **`01`**: Normal / Idle / Relative Flow
* **`10`**: Hovered / Focused / Absolute Flow
* **`11`**: Active / Pressed / Expanded Flow

Using `QuatByte`, **4 quaternary states are packed into a single standard byte (`u8`)**, ensuring ultra-high cache locality and minimal memory usage.

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
quat_ui = "0.3.0"
```

---

## 🚀 Quick Start (Rust)

Here is a quick example showing how to initialize a container widget, add child elements, and handle state changes:

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

    // 4. Simulate a focus event translated to a quaternary state
    let event = UiEvent::FocusGained;
    root_panel.set_state(UiState::new(event.target_quat()));
    
    println!("\n--- Render After Focus Event ---");
    root_panel.render();
}
```

---

## 🌐 FFI Integration (C# / Python)

Thanks to the built-in C-compatible FFI layer (`src/ffi.rs`), you can compile `quat_ui` into a dynamic library (`.dll`, `.so`, or `.dylib`) and use it in UI frameworks across other languages.

### C# (.NET) Example:
```csharp
using System;
using System.Runtime.InteropServices;

class Program
{
    [DllImport("quat_ui", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr quat_widget_new(string id);

    [DllImport("quat_ui", CallingConvention = CallingConvention.Cdecl)]
    private static extern void quat_widget_free(IntPtr widget);

    [DllImport("quat_ui", CallingConvention = CallingConvention.Cdecl)]
    private static extern void quat_widget_set_state(IntPtr widget, byte quatVal);

    [DllImport("quat_ui", CallingConvention = CallingConvention.Cdecl)]
    private static extern void quat_widget_render(IntPtr widget);

    static void Main()
    {
        IntPtr btn = quat_widget_new("SubmitButton");
        quat_widget_set_state(btn, 0b11); // Set state to Active (11)
        quat_widget_render(btn);
        quat_widget_free(btn);
    }
}
```

### Python Example:
```python
import ctypes

# Load the compiled library
lib = ctypes.CDLL("./target/release/libquat_ui.so")

# Setup argument and return types
lib.quat_widget_new.argtypes = [ctypes.c_char_p]
lib.quat_widget_new.restype = ctypes.c_void_p
lib.quat_widget_set_state.argtypes = [ctypes.c_void_p, ctypes.c_uint8]
lib.quat_widget_render.argtypes = [ctypes.c_void_p]
lib.quat_widget_free.argtypes = [ctypes.c_void_p]

# Usage
widget = lib.quat_widget_new(b"PythonButton")
lib.quat_widget_set_state(widget, 2)  # Binary 10 (Hover state)
lib.quat_widget_render(widget)
lib.quat_widget_free(widget)
```

---

## 🧪 Running Tests

QuatUI includes a comprehensive suite of unit tests ranging from basic bit manipulation to heavy stress tests with 10,000+ nodes in the tree buffer.

Run tests using Cargo:
```bash
cargo test
```