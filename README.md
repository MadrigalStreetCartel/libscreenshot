# libscreenshot
> A cross-platform library for screenshot creation.

Features:
- Easy cross-platform creation of screenshots
- Supports several types of screenshots
  - Window capture (focused window, by window id)
  - Screen capture (current screen, by screen id)
  - Full capture (all screens and windows)
  - Area capture

Limitations:
- Varying degrees of support for specific platforms (see `Platform Support`)
- For window capture, the window will be captured without the title area (except on macOS)

We are actively working on removing all of those limitations.

## Platform Support

| Platform | Window | Area | Screen | Full |
| -------- | ------------- | ----------- | ------------- | ----------- |
| Windows  | ✅            | ❌           | ❌            | ❌          |
| Linux    | ✅            | ✅           | ❌            | ✅          |
| macOS    | ✅            | ❌           | ✅            | ❌          |

## Usage

### Installation 

Add this to your `Cargo.toml`:

```toml
[dependencies]
libscreenshot = { git = "https://github.com/MadrigalStreetCartel/libscreenshot" }
```

### Features
> All features are enabled by default.

- `windows`
  - `windows_gdi`
  - `windows_graphics_capture`: requires Windows 10 Build 1803 or later
- `linux`
  - `linux_xorg`: requires X11
  - `linux_wayland`: requires Wayland
- `macos`

### Examples

**Capture focused window**:
```rust
let provider = libscreenshot::get_window_capture_provider().expect("Unable to find provider");
let image = provider.capture_focused_window().expect("Unable to capture focused window");
image.save("screenshot.png").expect("Unable to save image");
```

**Capture current screen**:
```rust
let provider = libscreenshot::get_screen_capture_provider().expect("Unable to find provider");
let image = provider.capture_current_screen().expect("Unable to capture screen");
image.save("screenshot.png").expect("Unable to save image");
```
