use std::path::PathBuf;

use super::raylib_api::RayLibApiDefinition;

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
            let mut name = String::new();
            for part in name_parts.iter() {
                let mut chars = part.chars();
                name.push(chars.next().unwrap_or(' ').to_ascii_uppercase());
                name.push_str(chars.as_str());
            }

            // Write the variant declaration
            lines.push(format!("\t{} = {},", name, variant.value));
        }

        // Close the enum declaration
        lines.push("}".to_string());

    }

    // Write the output file
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let output = lines.join("\n");
    std::fs::write(out_path.join("enums.rs"), output).unwrap();

}
