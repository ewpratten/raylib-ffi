use std::ffi::CString;

pub fn main() {
    unsafe {
        // Create a window
        raylib_ffi::InitWindow(
            800,
            450,
            CString::new("raylib-ffi example - basic window").unwrap().as_ptr(),
        );

        // Render the window
        while !(raylib_ffi::WindowShouldClose()) {
            raylib_ffi::draw!({
                // Render text and a background
                raylib_ffi::ClearBackground(raylib_ffi::colors::WHITE);
                raylib_ffi::DrawText(
                    CString::new("Congrats! You created your first window!").unwrap().as_ptr(),
                    190,
                    200,
                    20,
                    raylib_ffi::colors::BLACK,
                );
            });
        }

        // Clean up
        raylib_ffi::CloseWindow();
    }
}
