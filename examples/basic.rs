pub fn main() {
    unsafe {
        // Create a window
        raylib_ffi::InitWindow(
            800,
            450,
            raylib_ffi::rl_str!("raylib-ffi example - basic window"),
        );

        // Render the window
        while !(raylib_ffi::WindowShouldClose()) {
            raylib_ffi::draw!({
                // Render text and a background
                raylib_ffi::ClearBackground(raylib_ffi::colors::WHITE);
                raylib_ffi::DrawText(
                    raylib_ffi::rl_str!("Congrats! You created your first window!"),
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
