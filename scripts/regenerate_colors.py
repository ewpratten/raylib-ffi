import json
from pathlib import Path
import re

REPO_ROOT = Path(__file__).parent.parent
COLOR_DEF_RE = re.compile(r"CLITERAL\(Color\){ (\d+), (\d+), (\d+), (\d+) }")

print("Searching for raylib API definitions")
with open(REPO_ROOT / "third_party" / "raylib" / "parser" / "output" /
          "raylib_api.json") as f:
    raylib_api = json.load(f)

# Find the raylib defines
raylib_defines = raylib_api["defines"]

# Delete the old colors file if it exists
if (REPO_ROOT / "src" / "colors.rs").exists():
    print("Deleting old colors file")
    (REPO_ROOT / "src" / "colors.rs").unlink()

# Open the Rust colors file
with open(REPO_ROOT / "src" / "colors.rs", "w") as f:

    # Write a file header
    f.writelines([
        "//! This module contains auto-generated Rust representations of raylib's colors.\n"
    ])

    # Search for color definitions
    for definition in raylib_defines:
        if definition["type"] == "COLOR":
            print(f"Writing color: {definition['name']}")
            # Parse the RGBA values
            match = COLOR_DEF_RE.match(definition["value"])
            if match is None:
                raise RuntimeError(
                    f"Failed to parse color definition {definition}")
            r, g, b, a = match.groups()

            # Write the color definition
            f.writelines([
                "\n", f"/// {definition['description']}\n",
                f"pub const {definition['name']}: crate::Color = crate::Color {{\n\tr: {r},\n\tg: {g},\n\tb: {b},\n\ta: {a},\n}};\n"
            ])
