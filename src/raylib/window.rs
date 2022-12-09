use cgmath::Vector2;

use super::ffi as raylib;
use super::logging::rl_use_rust_logging;

lazy_static::lazy_static! {
    static ref WINDOW_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
}

/// A window handle
#[derive(Debug)]
pub struct Window {}

impl Window {
    /// Construct a new `Window`
    pub fn new(size: Vector2<i32>, title: &str) -> Self {
        // Make sure we only have one window
        if WINDOW_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst) > 0 {
            panic!("Only one window is allowed at a time");
        }

        // Perform FFI work
        unsafe {
            // Hook our rusty logging system into raylib
            rl_use_rust_logging();

            // Set some reasonable defaults
            raylib::SetWindowState(
                raylib::ConfigFlags_FLAG_VSYNC_HINT
                    | raylib::ConfigFlags_FLAG_WINDOW_RESIZABLE
                    | raylib::ConfigFlags_FLAG_MSAA_4X_HINT,
            );

            // Create a window
            raylib::InitWindow(
                size.x as i32,
                size.y as i32,
                format!("{}\0", title).as_ptr() as *const i8,
            );

        }

        Self {}
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        // Perform FFI work
        unsafe {
            // Clean up
            crate::raylib::ffi::CloseWindow();
        }

        // Decrement the window count
        WINDOW_COUNT.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
}
