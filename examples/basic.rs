use renderkit::raylib::window::Window;

pub fn main() {
    unsafe {
        // Set up logging
        env_logger::init();

        // Create a window
        let window = Window::new(
            cgmath::Vector2::new(800, 450),
            "raylib-ffi example - basic window",
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
                cstr::cstr!("Congrats! You created your first window!").as_ptr(),
                190,
                200,
                20,
                renderkit::raylib::palette::BLACK.into(),
            );

            // End the draw call
            renderkit::raylib::ffi::EndDrawing();
        }
    }
}
