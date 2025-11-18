# Debug Documentation

## Debug Code Management

This project uses a Python script to toggle debug code on/off dynamically.

### Quick Commands

```bash
# Enable debug code
just debug-enable
# or
python3 sys/rust/debug.py enable

# Disable debug code
just debug-disable
# or
python3 sys/rust/debug.py disable

# Check debug status
just debug-status
# or
python3 sys/rust/debug.py status
```

### Debug Marker Format

Debug code blocks are marked with `[DEBUG_START]` and `[DEBUG_END]` markers:

```rust
// [DEBUG_START] Description of debug block
// debug_code_line_1();
// debug_code_line_2();
// [DEBUG_END]
```

### Files with Debug Code

- `frontend/src/theme/loader.rs` - Theme loading diagnostics
- `frontend/src/state/app.rs` - Theme switching diagnostics

### How It Works

1. The script scans for `[DEBUG_START]` and `[DEBUG_END]` markers
2. Code between markers is toggled (commented/uncommented)
3. Original indentation is preserved
4. Works dynamically - add new debug blocks anywhere with the markers

**Example removal:**
```rust
// Before:
pub fn load_theme_by_name(name: &str) -> Result<ThemeConfig, String> {
    // [DEBUG] -> REMOVE LATER
    // DEBUG: Uncomment for theme loading diagnostics
    // web_sys::console::log_1(...);

    parse_theme_toml(toml_content)
}

// After:
pub fn load_theme_by_name(name: &str) -> Result<ThemeConfig, String> {
    parse_theme_toml(toml_content)
}
```

### How to Enable Debug Logs (During Development)

Uncomment the debug sections to see detailed console output:

1. Open browser DevTools (F12)
2. Go to Console tab
3. Uncomment debug lines in the code
4. Rebuild: `trunk build --release`
5. Refresh browser and test

Debug logs will show:
- Available themes list
- Which theme is being loaded
- Success/failure of theme loading
- Success/failure of theme parsing
