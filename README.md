# libscreenshot
> A cross-platform library for screenshot creation.

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
let provider libscreenshot::get_window_capture_provider().expect("Unable to find provider");
let image = provider.capture_current_screen().expect("Unable to capture screen");
image.save("screenshot.png").expect("Unable to save image");
```

**Capture current screen**:
```rust
let provider libscreenshot::get_screen_capture_provider().expect("Unable to find provider");
let image = provider.capture_current_screen().expect("Unable to capture screen");
image.save("screenshot.png").expect("Unable to save image");
```
