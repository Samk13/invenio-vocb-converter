# Invenio Controlled Vocabulary Converter

Converts JSON data dumps into Invenio YAML format for multiple controlled vocabularies.

## Installation

- Install Rust.

- Clone the repository.

- Build with:

```bash
cargo build --release
```

## Usage

```bash
./target/release/invenio-vocb-converter <VOCAB_TYPE> <INPUT_JSON> <OUTPUT_YAML>
```

VOCAB_TYPE: One of: `affiliations`, `names`, `funding`, `awards`, or `subjects`.

INPUT_JSON: Path to the JSON input file.

OUTPUT_YAML: Path for the generated YAML output.



## Example

```bash
./target/release/invenio-vocb-converter affiliations data/ror-data.json output/affiliations.yaml
```

## Error Handling
The converter validates arguments and reports errors for:

Missing or invalid parameters

File access issues

JSON parsing or YAML serialization errors

## License
Licensed under the MIT License. See LICENSE for details.
```