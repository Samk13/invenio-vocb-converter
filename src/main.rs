//! Controlled Vocabulary Converter
//!
//! Usage:
//!   vocab_converter <VOCAB_TYPE> <INPUT_JSON> <OUTPUT_YAML>
//!
//! VOCAB_TYPE can be one of:
//!   affiliations  - converts affiliations (e.g. from a ROR dump)
//!   names         - converts names
//!   funding       - converts funding records
//!   awards        - converts awards information
//!   subjects      - converts subject data
//!
//! Example:
//!   vocab_converter affiliations ./input.json ./output.yaml

use std::env;
use std::process;

mod vocab {
    /// Module for converting an Affiliations vocabulary.
    pub mod affiliations {
        use deunicode::deunicode;
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;
        use std::error::Error;
        use std::fs::File;
        use std::io::{BufReader, BufWriter, Write};
        use std::path::Path;

        #[derive(Debug, Deserialize)]
        pub struct AffiliationItem {
            #[serde(deserialize_with = "deserialize_null_default")]
            pub id: String,
            #[serde(deserialize_with = "deserialize_null_default")]
            pub name: String,
            #[serde(default)]
            pub labels: Vec<Label>,
            #[serde(default)]
            pub acronyms: Vec<String>,
        }

        #[derive(Debug, Deserialize)]
        pub struct Label {
            #[serde(deserialize_with = "deserialize_null_default")]
            pub iso639: String,
            #[serde(deserialize_with = "deserialize_null_default")]
            pub label: String,
        }

        pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
        where
            T: Default + Deserialize<'de>,
            D: serde::Deserializer<'de>,
        {
            let opt = Option::deserialize(deserializer)?;
            Ok(opt.unwrap_or_default())
        }

        /// Sanitize a string by transliterating ambiguous Unicode characters (such as Cyrillic)
        /// into their approximate ASCII equivalents.
        fn sanitize(s: &str) -> String {
            deunicode(s)
        }

        #[derive(Debug, Serialize)]
        pub struct YamlEntry {
            pub id: String,
            pub name: String,
            pub title: HashMap<String, String>,
            pub identifiers: Vec<Identifier>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub acronym: Option<String>,
        }

        #[derive(Debug, Serialize)]
        pub struct Identifier {
            pub identifier: String,
            pub scheme: String,
        }

        /// Convert a JSON file containing Affiliations data into a YAML file.
        /// This function sanitizes all strings to replace ambiguous characters.
        pub fn convert_json_to_yaml(json_path: &Path, yaml_path: &Path) -> Result<(), Box<dyn Error>> {
            // Open and deserialize the JSON file.
            let file = File::open(json_path)?;
            let reader = BufReader::new(file);
            let items: Vec<AffiliationItem> = serde_json::from_reader(reader)?;

            let mut yaml_data = Vec::new();

            for item in items {
                // Sanitize the id and extract the last segment.
                let id_sanitized = sanitize(&item.id);
                let id_part = id_sanitized.split('/').last().unwrap_or_default().to_string();

                let mut title = std::collections::HashMap::new();
                title.insert("en".to_string(), sanitize(&item.name));

                // Process and sanitize any labels.
                for label in &item.labels {
                    if !label.iso639.is_empty() && !label.label.is_empty() {
                        title.insert(sanitize(&label.iso639), sanitize(&label.label));
                    }
                }

                // Get the first non-empty acronym, if available.
                let acronym = item.acronyms.iter()
                    .find(|s| !s.is_empty())
                    .map(|s| sanitize(s));

                let identifier = Identifier {
                    identifier: id_part.clone(),
                    scheme: "affiliation".to_string(),
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

            // Create file and wrap with BufWriter.
            let file = File::create(yaml_path)?;
            let mut writer = BufWriter::new(file);

            // Optionally, write the UTF-8 BOM to ensure proper encoding detection.
            writer.write_all(b"\xEF\xBB\xBF")?;

            // Serialize the data to YAML.
            serde_yaml::to_writer(&mut writer, &yaml_data)?;

            Ok(())
        }
    } // end of affiliations module

    // Placeholder modules for future controlled vocabularies.

    pub mod names {
        use std::error::Error;
        use std::path::Path;

        #[allow(dead_code)]
        pub fn convert_json_to_yaml(_json_path: &Path, _yaml_path: &Path) -> Result<(), Box<dyn Error>> {
            Err("Names vocabulary conversion not yet implemented.".into())
        }
    }

    pub mod funding {
        use std::error::Error;
        use std::path::Path;

        #[allow(dead_code)]
        pub fn convert_json_to_yaml(_json_path: &Path, _yaml_path: &Path) -> Result<(), Box<dyn Error>> {
            Err("Funding vocabulary conversion not yet implemented.".into())
        }
    }

    pub mod awards {
        use std::error::Error;
        use std::path::Path;

        #[allow(dead_code)]
        pub fn convert_json_to_yaml(_json_path: &Path, _yaml_path: &Path) -> Result<(), Box<dyn Error>> {
            Err("Awards vocabulary conversion not yet implemented.".into())
        }
    }

    pub mod subjects {
        use std::error::Error;
        use std::path::Path;

        #[allow(dead_code)]
        pub fn convert_json_to_yaml(_json_path: &Path, _yaml_path: &Path) -> Result<(), Box<dyn Error>> {
            Err("Subjects vocabulary conversion not yet implemented.".into())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "Usage: {} <VOCAB_TYPE> <INPUT_JSON> <OUTPUT_YAML>",
            args[0]
        );
        eprintln!("VOCAB_TYPE must be one of: affiliations, names, funding, awards, subjects");
        process::exit(1);
    }

    let vocab_type = &args[1].to_lowercase(); // normalize to lowercase
    let json_path = std::path::Path::new(&args[2]);
    let yaml_path = std::path::Path::new(&args[3]);

    match vocab_type.as_str() {
        "affiliations" => {
            vocab::affiliations::convert_json_to_yaml(json_path, yaml_path)?;
        }
        "names" => {
            eprintln!("Names vocabulary conversion not yet implemented.");
            process::exit(1);
        }
        "funding" => {
            eprintln!("Funding vocabulary conversion not yet implemented.");
            process::exit(1);
        }
        "awards" => {
            eprintln!("Awards vocabulary conversion not yet implemented.");
            process::exit(1);
        }
        "subjects" => {
            eprintln!("Subjects vocabulary conversion not yet implemented.");
            process::exit(1);
        }
        _ => {
            eprintln!("Unknown vocabulary type: {}", vocab_type);
            process::exit(1);
        }
    }

    Ok(())
}
