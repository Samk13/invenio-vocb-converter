//! ROR JSON to YAML Converter
//!
//! Usage:
//!   ror_converter <INPUT_JSON> <OUTPUT_YAML>
//!
//! Arguments:
//!   <INPUT_JSON>   Path to input JSON file (ROR data)
//!   <OUTPUT_YAML>  Path for output YAML file
//!
//! Example:
//!   ror_converter ./input.json ./output.yaml

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

// Import the deunicode crate for transliteration.
use deunicode::deunicode;

#[derive(Debug, Deserialize)]
struct RorItem {
    #[serde(deserialize_with = "deserialize_null_default")]
    id: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    name: String,
    #[serde(default)]
    labels: Vec<Label>,
    #[serde(default)]
    acronyms: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Label {
    #[serde(deserialize_with = "deserialize_null_default")]
    iso639: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    label: String,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// Sanitize a string by transliterating ambiguous Unicode characters (such as Cyrillic)
/// into their ASCII equivalents.
fn sanitize(s: &str) -> String {
    deunicode(s)
}

#[derive(Debug, Serialize)]
struct YamlEntry {
    id: String,
    name: String,
    title: HashMap<String, String>,
    identifiers: Vec<Identifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acronym: Option<String>,
}

#[derive(Debug, Serialize)]
struct Identifier {
    identifier: String,
    scheme: String,
}

fn convert_json_to_yaml(
    json_path: &Path,
    yaml_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read JSON file with UTF-8 encoding.
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    let items: Vec<RorItem> = serde_json::from_reader(reader)?;

    let mut yaml_data = Vec::new();

    for item in items {
        // Sanitize the id and extract the last segment.
        let id_sanitized = sanitize(&item.id);
        let id_part = id_sanitized.split('/').last().unwrap_or_default().to_string();

        let mut title = HashMap::new();
        // Sanitize the primary name and set it as the English title.
        title.insert("en".to_string(), sanitize(&item.name));

        // Process each label and sanitize both the ISO639 code and its label.
        for label in &item.labels {
            if !label.iso639.is_empty() && !label.label.is_empty() {
                title.insert(sanitize(&label.iso639), sanitize(&label.label));
            }
        }

        // Get the first non-empty acronym and sanitize it.
        let acronym = item.acronyms.iter()
            .find(|s| !s.is_empty())
            .map(|s| sanitize(s));

        let identifier = Identifier {
            identifier: id_part.clone(),
            scheme: "ror".to_string(),
        };

        let yaml_entry = YamlEntry {
            id: id_part,
            name: sanitize(&item.name),
            title,
            identifiers: vec![identifier],
            acronym,
        };

        yaml_data.push(yaml_entry);
    }

    // Create file and wrap in a BufWriter.
    let file = File::create(yaml_path)?;
    let mut writer = BufWriter::new(file);

    // Optionally, write the UTF-8 BOM (not strictly necessary, but sometimes helps editors detect encoding).
    writer.write_all(b"\xEF\xBB\xBF")?;
    
    // Serialize and write the YAML output.
    serde_yaml::to_writer(&mut writer, &yaml_data)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <INPUT_JSON> <OUTPUT_YAML>", args[0]);
        std::process::exit(1);
    }

    let json_path = Path::new(&args[1]);
    let yaml_path = Path::new(&args[2]);

    convert_json_to_yaml(json_path, yaml_path)
}
