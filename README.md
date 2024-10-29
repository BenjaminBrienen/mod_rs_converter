# mod_rs_converter

`mod_rs_converter` is a Rust CLI tool that automates the migration from the `mod.rs` module file structure to Rust’s new recommended module layout. It scans a Rust project's directory for `mod.rs` files, renames them according to the containing folder name, and moves them up one level.

## Overview

Rust has shifted its module structure recommendations away from using `mod.rs` files. This tool renames and moves `mod.rs` files to follow the new convention, making your project structure cleaner and more modern.

### Example

- **Before**:
  `/example_crate/example_module/mod.rs`

- **After**:
  `/example_crate/example_module.rs`

## Installation

First, clone this repository and navigate into it:

```sh
git clone <repository-url>
cd mod_rs_converter
```

Then, build the project:

```sh
cargo build --release
```

This will generate an executable located in `target/release/`.

## Usage

Run the tool with the path to your Rust project as an argument:

```sh
./target/release/mod_rs_converter /path/to/your/rust/project
```

The tool will traverse the specified directory, locate `mod.rs` files, rename and move them according to the updated layout.

## How It Works

1. **Traverse Project Structure**: Scans the directory for `mod.rs` files.
2. **Determine New Path**: Renames each `mod.rs` file to `<parent_folder>.rs` and moves it up one directory level.
3. **Log Output**: Provides feedback in the terminal for each file it processes.

### Notes

- If a file with the new name already exists in the target location, the tool will skip that file and log a warning to avoid overwriting.
- The current version of this tool does **not** update `mod` declarations in source code files. You may need to manually update these after running the tool.

## Example Output

When run, the tool will display messages similar to:

```sh
Renaming and moving /example_crate/example_module/mod.rs to /example_crate/example_module.rs
Renaming and moving /example_crate/another_module/mod.rs to /example_crate/another_module.rs
Warning: Destination file /example_crate/existing_module.rs already exists. Skipping.
```

## License

This project is licensed under dual Apache and MIT License.
