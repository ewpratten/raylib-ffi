use raylib_ffi::*;
use ::std::os::raw::*;
use std::ffi::CString;

const MAX_LIGHTS: usize = 4;    // Max dynamic lights supported by shader

// Light data
struct Light {
    enabled: i32,
    kind: i32,
    position: Vector3,
    target: Vector3,
    color: Color,

    // Shader locations
    enabled_loc: i32,
    kind_loc: i32,
    position_loc: i32,
    target_loc: i32,
    color_loc: i32,
}

// Create a light and get shader locations
fn create_light(id: isize, kind: i32, position: Vector3, target : Vector3, color : Color, shader : Shader) -> Light {
    unsafe {
        let light = Light {
            enabled: 1,
            kind,
            position,
            target,
            color,

            // NOTE: Lighting shader naming must be the provided ones
            enabled_loc: GetShaderLocation(shader, CString::new(format!("lights[{}].enabled", id)).unwrap().as_ptr()),
            kind_loc: GetShaderLocation(shader, CString::new(format!("lights[{}].type", id)).unwrap().as_ptr()),
            position_loc: GetShaderLocation(shader, CString::new(format!("lights[{}].position", id)).unwrap().as_ptr()),
            target_loc: GetShaderLocation(shader, CString::new(format!("lights[{}].target", id)).unwrap().as_ptr()),
            color_loc: GetShaderLocation(shader, CString::new(format!("lights[{}].color", id)).unwrap().as_ptr())
        };

        update_light_values(shader, &light);
        return light;
    }
}

// Send light properties to shader
// NOTE: Light shader locations should be available
fn update_light_values(shader: Shader, light: &Light) {
    unsafe {
        // Send to shader light enabled state and type
        let enabled = [light.enabled].as_ptr();
        SetShaderValue(shader, light.enabled_loc, enabled as *const c_void, enums::ShaderUniformDataType::Int as i32);
        let kind = [light.kind].as_ptr();
        SetShaderValue(shader, light.kind_loc, kind as *const c_void, enums::ShaderUniformDataType::Int as i32);

        // Send to shader light position values
        let position = [light.position.x, light.position.y, light.position.z].as_ptr();
        SetShaderValue(shader, light.position_loc, position as *const c_void, enums::ShaderUniformDataType::Vec3 as i32);

        // Send to shader light target position values
        let target = [light.position.x, light.position.y, light.position.z].as_ptr();
        SetShaderValue(shader, light.target_loc, target as *const c_void, enums::ShaderUniformDataType::Vec3 as i32);

        // Send to shader light color values
        let color = [
            light.color.r as f32 / 255.0,
            light.color.g as f32 / 255.0,
            light.color.b as f32 / 255.0,
            light.color.a as f32 / 255.0
        ].as_ptr();
        SetShaderValue(shader, light.color_loc, color as *const c_void, enums::ShaderUniformDataType::Vec4 as i32);
    }
}

//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------
pub fn main() {
    unsafe {
        // Initialization
        //--------------------------------------------------------------------------------------
        SetConfigFlags(enums::ConfigFlags::Msaa4xHint as u32);  // Enable Multi Sampling Anti Aliasing 4x (if available)
        InitWindow(800, 450, CString::new("raylib [shaders] example - basic lighting").unwrap().as_ptr());

        // Define the camera to look into our 3d world
        let mut camera = Camera{
            position: Vector3{ x: 2.0, y: 4.0, z: 6.0 },            // Camera position
            target: Vector3{ x: 0.0, y: 0.5, z: 0.0 },              // Camera looking at point     
            up: Vector3{ x: 0.0, y: 1.0, z: 0.0 },                  // Camera up vector (rotation towards target)
            fovy: 45.0,                                             // Camera field-of-view Y
            projection: enums::CameraProjection::Perspective as i32 // Camera projection type
        };

        // Load plane model from a generated mesh
        let model = LoadModelFromMesh(GenMeshPlane(10.0, 10.0, 3, 3));
        let cube = LoadModelFromMesh(GenMeshCube(2.0, 4.0, 2.0));
        
        // Load basic lighting shader
        let shader = LoadShader(CString::new("examples/shaders/lighting.vs").unwrap().as_ptr(), CString::new("examples/shaders/lighting.fs").unwrap().as_ptr());

        // Get some required shader locations
        let view_loc = shader.locs.offset(enums::ShaderLocationIndex::VectorView as isize) as *mut c_int;
        *view_loc = GetShaderLocation(shader, CString::new("viewPos").unwrap().as_ptr());

        // Ambient light level (some basic lighting)
        let ambient_loc = GetShaderLocation(shader, CString::new("ambient").unwrap().as_ptr());
        let ambient_value = [0.1 as f32, 0.1 as f32, 0.1 as f32, 1.0 as f32].as_ptr();
        SetShaderValue(shader, ambient_loc, ambient_value as *const c_void, enums::ShaderUniformDataType::Ivec4 as i32);

        // Assign lighting shader to model
        (*(model.materials.offset(0))).shader = shader;
        (*(cube.materials.offset(0))).shader = shader;

        // Create lights
        let mut lights = [
            create_light(0, 1, Vector3{ x: -2.0, y: 1.0, z: -2.0 }, Vector3{ x: 0.0, y: 0.0, z: 0.0 }, colors::YELLOW, shader),
            create_light(1, 1, Vector3{ x: 2.0, y: 1.0, z: 2.0 }, Vector3{ x: 0.0, y: 0.0, z: 0.0 }, colors::RED, shader),
            create_light(2, 1, Vector3{ x: -2.0, y: 1.0, z: 2.0 }, Vector3{ x: 0.0, y: 0.0, z: 0.0 }, colors::GREEN, shader),
            create_light(3, 1, Vector3{ x: 2.0, y: 1.0, z: -2.0 }, Vector3{ x: 0.0, y: 0.0, z: 0.0 }, colors::BLUE, shader)
        ];

        SetTargetFPS(60);               // Set our game to run at 60 frames-per-second
        //--------------------------------------------------------------------------------------

        // Main game loop
        while !WindowShouldClose()          // Detect window close button or ESC key
        {
            // Update
            //----------------------------------------------------------------------------------
            UpdateCamera(&mut camera, enums::CameraMode::Orbital as i32);

            // Update the shader with the camera view vector (points towards { 0.0f, 0.0f, 0.0f })
            let camera_pos = [camera.position.x, camera.position.y, camera.position.z].as_ptr();
            SetShaderValue(shader, shader.locs.offset(enums::ShaderLocationIndex::VectorView as isize).read(), camera_pos as *mut c_void, enums::ShaderUniformDataType::Ivec3 as c_int);

            // Check key inputs to enable/disable lights
            if IsKeyPressed(enums::KeyboardKey::R as i32) { lights[1].enabled = !lights[1].enabled; }
            if IsKeyPressed(enums::KeyboardKey::G as i32) { lights[2].enabled = !lights[2].enabled; }
            if IsKeyPressed(enums::KeyboardKey::B as i32) { lights[3].enabled = !lights[3].enabled; }
            if IsKeyPressed(enums::KeyboardKey::Y as i32) { lights[0].enabled = !lights[0].enabled; }

            // Update light values (actually, only enable/disable them)
            for i in 0 .. MAX_LIGHTS { 
                update_light_values(shader, &lights[i]);
            }
            //----------------------------------------------------------------------------------

            // Draw
            //----------------------------------------------------------------------------------
            BeginDrawing();

                ClearBackground(colors::WHITE);

                BeginMode3D(camera);

                    DrawModel(model, Vector3{ x: 0.0, y: 0.0, z: 0.0 }, 1.0, colors::WHITE);
                    DrawModel(cube, Vector3{ x: 0.0, y: 0.0, z: 0.0 }, 1.0, colors::WHITE);

                    // Draw spheres to show where the lights are
                    for i in 0 .. MAX_LIGHTS {
                        if lights[i].enabled > 0 {
                            DrawSphereEx(lights[i].position, 0.2, 8, 8, lights[i].color);
                        } else {
                            DrawSphereWires(lights[i].position, 0.2, 8, 8, ColorAlpha(lights[i].color, 0.3));
                        }
                    }

                    DrawGrid(10, 1.0);

                EndMode3D();

                DrawFPS(10, 10);

                DrawText(CString::new("Use keys [Y][R][G][B] to toggle lights").unwrap().as_ptr(), 10, 40, 20, colors::DARKGRAY);

            EndDrawing();
            //----------------------------------------------------------------------------------
        }

        // De-Initialization
        //--------------------------------------------------------------------------------------
        UnloadModel(model);     // Unload the model
        UnloadModel(cube);      // Unload the model
        UnloadShader(shader);   // Unload shader

        CloseWindow();          // Close window and OpenGL context
        //--------------------------------------------------------------------------------------
    }
}
