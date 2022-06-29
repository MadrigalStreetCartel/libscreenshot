# libscreenshot
> A cross-platform library for screenshot creation.

## Platform Support

| Platform | Features        |
| -------- | --------------- |
| Windows  | `WindowCapture` |
| Linux    | `WindowCapture` |
| macOS    | None            |

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

```rust
let provider = libscreenshot::get_best_window_capture_provider();
if let Ok(image) = provider.capture_focused_window() {
  image.save_to_file("screenshot.png").unwrap();
}
```