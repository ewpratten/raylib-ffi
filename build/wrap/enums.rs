use std::path::PathBuf;

use super::raylib_api::RayLibApiDefinition;

/// A mapping of enum names to the number of tokens to pop off the variant prefix
/// (C enums tend to have prefixes to avoid name collisions)
fn get_prefix_len(name: &str) -> usize {
    // Some enums have more than one prefix token
    match name {
        "CubemapLayout" => 2,
        "GamepadAxis" => 2,
        "GamepadButton" => 2,
        "MaterialMapIndex" => 2,
        "MouseButton" => 2,
        "MouseCursor" => 2,
        "PixelFormat" => 2,
        "ShaderAttributeDataType" => 2,
        "ShaderLocationIndex" => 2,
        "ShaderUniformDataType" => 2,
        "TextureFilter" => 2,
        "TextureWrap" => 2,
        _ => 1,
    }
}

/// Gets a list of enum names for bindgen to skip.
/// We are manually binding them to Rust here, so there is no need to double-translate
pub fn get_blocked_enum_names() -> Vec<String> {
    vec![
        "BlendMode",
        "CameraMode",
        "CameraProjection",
        "ConfigFlags",
        "CubemapLayout",
        "FontType",
        "GamepadAxis",
        "GamepadButton",
        "Gesture",
        "KeyboardKey",
        "MaterialMapIndex",
        "MouseButton",
        "MouseCursor",
        "NPatchLayout",
        "PixelFormat",
        "ShaderAttributeDataType",
        "ShaderLocationIndex",
        "ShaderUniformDataType",
        "TextureFilter",
        "TextureWrap",
        "TraceLogLevel"
    ].into_iter().map(|s| s.to_string()).collect()
}

pub fn wrap_exposed_enums(api_defs: RayLibApiDefinition) {
    // Allocate an output buffer for lines
    let mut lines = Vec::new();

    // Handle each enum
    for en in api_defs.enums {
        // Write a doc comment with raylib's provided enum description
        lines.push("".to_string());
        lines.push(format!("/// {}", en.description));

        // Write the enum declaration
        lines.push("#[repr(C)]".to_string());
        lines.push("#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]".to_string());
        lines.push(format!("pub enum {} {{", en.name));

        // Write each enum variant
        for variant in en.values {
            // For now, we have to skip `KEY_MENU` because it shares a value with `KEY_R`
            if variant.name == "KEY_MENU" {
                continue;
            }

            // Write a doc comment with raylib's provided variant description
            lines.push(format!("\t/// {}", variant.description));

            // Transform the name into a valid Rust identifier
            let name_parts = variant.name.split('_').collect::<Vec<_>>();
            // Ignore the prefix tokens
            let name_parts = &name_parts[get_prefix_len(&en.name)..];
            // Capitalize and join
            let mut name = String::new();
            for part in name_parts.iter() {
                let mut chars = part.chars();
                name.push(chars.next().unwrap_or(' ').to_ascii_uppercase());
                name.push_str(chars.as_str().to_ascii_lowercase().as_str());
            }

            // Write the variant declaration
            lines.push(format!("\t{} = {},", name, variant.value));
        }

        // Close the enum declaration
        lines.push("}".to_string());

        // Write a conversion function
        lines.push(format!("impl Into<usize> for {} {{", en.name));
        lines.push(format!("\tfn into(self) -> usize {{"));
        lines.push(format!("\t\tself as usize"));
        lines.push(format!("\t}}"));
        lines.push(format!("}}"));
    }

    // Write the output file
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let output = lines.join("\n");
    std::fs::write(out_path.join("enums.rs"), output).unwrap();
}
