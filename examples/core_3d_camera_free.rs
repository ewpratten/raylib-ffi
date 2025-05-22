use raylib_ffi::*;
use std::ffi::CString;

pub fn main() {
    unsafe {
        // Create a window
        InitWindow(800, 450, CString::new("raylib-ffi example - basic window").unwrap().as_ptr());

        // Define the camera to look into our 3d world (position, target, up vector)
        let mut camera = Camera{
            position: Vector3{ x: 10.0, y: 10.0, z: 10.0 },         // Camera position
            target: Vector3{ x: 0.0, y: 0.0, z: 0.0 },              // Camera looking at point     
            up: Vector3{ x: 0.0, y: 1.0, z: 0.0 },                  // Camera up vector (rotation towards target)
            fovy: 45.0,                                             // Camera field-of-view Y
            projection: enums::CameraProjection::Perspective as i32 // Camera projection type
        };

        let cube_position = Vector3{ x: 0.0, y: 0.0, z: 0.0 };

        DisableCursor();                // Limit cursor to relative movement inside the window

        SetTargetFPS(60);               // Set our game to run at 60 frames-per-second

        while !(WindowShouldClose()) {  // Detect window close button or ESC key

            // Update
            //----------------------------------------------------------------------------------
            UpdateCamera(&mut camera, enums::CameraMode::Free as i32);

            if IsKeyPressed(enums::KeyboardKey::Z as i32) {
                camera.target = Vector3{ x: 0.0, y: 0.0, z: 0.0 };
            }
            //----------------------------------------------------------------------------------

            // Draw
            //----------------------------------------------------------------------------------
            BeginDrawing();

                ClearBackground(colors::WHITE);

                BeginMode3D(camera);

                    DrawCube(cube_position, 2.0, 2.0, 2.0, colors::RED);
                    DrawCubeWires(cube_position, 2.0, 2.0, 2.0, colors::MAROON);
                    DrawGrid(10, 1.0);

                EndMode3D();

                DrawRectangle( 10, 10, 320, 133, Fade(colors::SKYBLUE, 0.5));
                DrawRectangleLines( 10, 10, 320, 133, colors::BLUE);
    
                DrawText(CString::new("Free camera default controls:").unwrap().as_ptr(), 20, 20, 10, colors::BLACK);
                DrawText(CString::new("- Mouse Wheel to Zoom in-out").unwrap().as_ptr(), 40, 40, 10, colors::DARKGRAY);
                DrawText(CString::new("- Mouse Wheel Pressed to Pan").unwrap().as_ptr(), 40, 60, 10, colors::DARKGRAY);
                DrawText(CString::new("- Alt + Mouse Wheel Pressed to Rotate").unwrap().as_ptr(), 40, 80, 10, colors::DARKGRAY);
                DrawText(CString::new("- Alt + Ctrl + Mouse Wheel Pressed for Smooth Zoom").unwrap().as_ptr(), 40, 100, 10, colors::DARKGRAY);
                DrawText(CString::new("- Z to zoom to (0, 0, 0)").unwrap().as_ptr(), 40, 120, 10, colors::DARKGRAY);

            EndDrawing();
            //----------------------------------------------------------------------------------
        }

        // De-Initialization
        //--------------------------------------------------------------------------------------
        CloseWindow();
        //--------------------------------------------------------------------------------------
    }
}
