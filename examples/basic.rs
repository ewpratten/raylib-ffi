pub fn main() {
    unsafe {
        // Create a window
        renderkit::raylib::ffi::InitWindow(
            800,
            450,
            "raylib-ffi example - basic window\0".as_ptr() as *const i8,
        );

        // Render the window
        loop {
            // Close the window if requested
            if renderkit::raylib::ffi::WindowShouldClose() {
                break;
            }

            // Begin a draw call
            renderkit::raylib::ffi::BeginDrawing();

            // Render text and a background
            renderkit::raylib::ffi::ClearBackground(renderkit::raylib::palette::RAYWHITE.into());
            renderkit::raylib::ffi::DrawText(
                "Congrats! You created your first window!\0".as_ptr() as *const i8,
                190,
                200,
                20,
                renderkit::raylib::palette::BLACK.into(),
            );

            // End the draw call
            renderkit::raylib::ffi::EndDrawing();
        }

        // Clean up
        renderkit::raylib::ffi::CloseWindow();
    }
}
