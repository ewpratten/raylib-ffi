import json
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent

print("Searching for raylib API definitions")
with open(REPO_ROOT / "third_party" / "raylib" / "parser" / "output" /
          "raylib_api.json") as f:
    raylib_api = json.load(f)

# Find the raylib enums
raylib_enums = raylib_api["enums"]

# Delete the old enums file if it exists
if (REPO_ROOT / "src" / "enums.rs").exists():
    print("Deleting old enums file")
    (REPO_ROOT / "src" / "enums.rs").unlink()

# Open the Rust enums file
with open(REPO_ROOT / "src" / "enums.rs", "w") as f:

    # Write a file header
    f.writelines([
        "//! This module contains auto-generated Rust representations of raylib's enums.\n"
    ])

    # Translate each found enum
    for enum in raylib_enums:
        print(f"Translating enum {enum['name']}")
        # Write a doc comment and the enum header
        f.writelines([
            "\n", f"/// {enum['description']}\n",
            "#[repr(C)]\n",
            "#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]\n",
            f"pub enum {enum['name']} {{\n"
        ])

        # Write each enum variant
        for variant in enum["values"]:
            # TODO: I'm not sure how best to handle duplicate variant values
            if variant["name"] == "KEY_MENU":
                continue
            
            # Normalize the variant name
            variant_name = "".join([
                segment.capitalize() for segment in variant["name"].split("_")
            ])

            # Write
            f.writelines([
                f"\t/// {variant['description']}\n",
                f"\t{variant_name} = {variant['value']},\n"
            ])

        # Write the enum footer
        f.writelines(["}\n"])
