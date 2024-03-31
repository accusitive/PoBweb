import os

# Set the root directory
root_dir = "PathOfBuilding/src"

# Function to generate the Rust module content
def generate_rust_module():
    module_content = "pub fn get_lua_source(path: &str) -> &'static str {\n"
    module_content += "    match path {\n"

    # Iterate over all files in the root directory and subdirectories
    for root, dirs, files in os.walk(root_dir):
        for file in files:
            # Check if the file is a .lua file
            if file.endswith(".lua"):
                # Get the relative path of the file
                rel_path = os.path.relpath(os.path.join(root, file), root_dir)
                # Replace backslashes with forward slashes (for Windows compatibility)
                rel_path = rel_path.replace("\\", "/")
                # Remove the .lua extension from the file name
                key = rel_path[:-4]
                # Append the match arm to the module content
                module_content += f'        "{key}" => include_str!("../{root_dir}/{rel_path}"),\n'

    module_content += '        _ => unreachable!(),\n'
    module_content += '    }\n'
    module_content += '}\n'

    return module_content

# Write the module content to src/lua_sources.rs
with open("src/lua_sources.rs", "w", encoding="utf-8") as file:
    file.write(generate_rust_module())