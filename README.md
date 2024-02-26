# Filter Org Txt

This Rust project filters an input text file by applying patterns from a `patterns.toml` file.

## Usage

```sh
cargo run <input_file_path> [output_file_path]
```

- `input_file_path`: Path to the input text file that you want to filter.
- `output_file_path` (optional): Path to the output file where the filtered text will be stored. If not provided, the input file will be overwritten.

## Patterns Configuration

Create a `patterns.toml` file in the project root directory with the following structure:

```toml
patterns = [
    "pattern1",
    "pattern2",
    ...
]
```

Replace `pattern1`, `pattern2`, etc. with the desired patterns to apply to the input text file.

## Example

If you have the following `patterns.toml` file:

```toml
patterns = [
    "example",
    "test"
]
```

And the input text file contains the following text:

```
This is an example.
This is a test.
This is a demo.
```

The output will be:

```
This is an .
This is a .
This is a demo.
```

The patterns "example" and "test" were removed from the input text.
