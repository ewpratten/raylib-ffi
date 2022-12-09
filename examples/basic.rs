pub fn main() {
    unsafe {
        // Create a window
        raylib_ffi::InitWindow(
            800,
            450,
            "raylib-ffi example - basic window\0".as_ptr() as *const i8,
        );

        // Render the window
        loop {
            // Close the window if requested
            if raylib_ffi::WindowShouldClose() {
                break;
            }

            // Begin a draw call
            raylib_ffi::BeginDrawing();

            // Render text and a background
            raylib_ffi::ClearBackground(raylib_ffi::Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            });
            raylib_ffi::DrawText(
                "Congrats! You created your first window!\0".as_ptr() as *const i8,
                190,
                200,
                20,
                raylib_ffi::Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                },
            );

            // End the draw call
            raylib_ffi::EndDrawing();
        }

        // Clean up
        raylib_ffi::CloseWindow();
    }
}
