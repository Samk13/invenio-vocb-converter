# ROR Data Converter

Converts ROR JSON data files to YAML format.

## Installation

1. Install Rust: https://rustup.rs/
2. Clone repository
3. Build:
```bash
cargo build --release
```

4. Run:
```bash
cargo run --release -- <INPUT_JSON> <OUTPUT_YAML>
```

## Arguments

| Parameter    | Description                          | Example                     |
|--------------|--------------------------------------|-----------------------------|
| INPUT_JSON   | Path to ROR JSON input file         | ./data/ror-data.json       |
| OUTPUT_YAML  | Path for generated YAML output file  | ./output/affiliations.yaml  |


Example

```bash
cargo run --release -- ./data/ror-data.json ./output/affiliations.yaml
```

**Error Handling:**

The program will show appropriate errors for:
- Missing arguments
- Invalid file paths
- JSON parsing errors
- YAML serialization errors
- File permission issues

The code now properly validates arguments and provides clear error messages while maintaining all the original conversion functionality in a more Rust-idiomatic way.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.