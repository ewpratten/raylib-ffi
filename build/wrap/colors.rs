use std::path::PathBuf;

use regex::Regex;
use serde_json::Value;

use super::raylib_api::RayLibApiDefinition;

pub fn wrap_default_colors(api_defs: RayLibApiDefinition) {
    // Allocate an output buffer for lines
    let mut lines = Vec::new();

    // Compile the regex statement that fines color definitions
    let color_re = Regex::new(r"CLITERAL\(Color\)\{ (\d+), (\d+), (\d+), (\d+) \}").unwrap();

    // Handle each enum
    for def in api_defs.defines {
        // Only operate on color types
        if def.kind == "COLOR" {
            if let Value::String(value) = def.value {
                // Write a doc comment describing the color
                lines.push("".to_string());
                lines.push(format!("/// {}", def.description));

                // Extract the color components from the value
                let caps = color_re.captures(&value).unwrap();
                let r = caps.get(1).unwrap().as_str();
                let g = caps.get(2).unwrap().as_str();
                let b = caps.get(3).unwrap().as_str();
                let a = caps.get(4).unwrap().as_str();

                // Write the color declaration
                lines.push(format!(
                    "pub const {}: crate::Color = crate::Color {{\n\tr: {},\n\tg: {},\n\tb: {},\n\ta: {}\n}};",
                    def.name, r, g, b, a
                ));
            }
        }
    }

    // Write the output file
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let output = lines.join("\n");
    std::fs::write(out_path.join("colors.rs"), output).unwrap();
}
