use invenio_vocb_converter::vocab::affiliations;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_sanitize_strings() {
    // Test with Cyrillic characters
    assert_eq!(
        affiliations::sanitize("Московский государственный университет"), 
        "Moskovskii gosudarstvennyi universitet"
    );
    
    // Test with accented characters
    assert_eq!(
        affiliations::sanitize("Université de Genève"), 
        "Universite de Geneve"
    );
    
    // Test with German umlauts
    assert_eq!(
        affiliations::sanitize("Technische Universität München"), 
        "Technische Universitat Munchen"
    );
    
    // Test with special characters and symbols
    assert_eq!(
        affiliations::sanitize("Max-Planck-Institut für Physik"), 
        "Max-Planck-Institut fur Physik"
    );
}

#[test]
fn test_deserialize_null_default() {
    // Use serde_json's from_value to test the deserializer
    use serde_json::json;
    use serde::Deserialize;
    
    #[derive(Deserialize, Debug, PartialEq)]
    struct TestStruct {
        #[serde(deserialize_with = "affiliations::deserialize_null_default")]
        value: String,
    }
    
    // Test with a valid string value
    let json_with_value = json!({"value": "test"});
    let result: TestStruct = serde_json::from_value(json_with_value).unwrap();
    assert_eq!(result.value, "test");
    
    // Test with a null value
    let json_with_null = json!({"value": null});
    let result: TestStruct = serde_json::from_value(json_with_null).unwrap();
    assert_eq!(result.value, "");
    
    // Test with a missing field - this is expected to fail with default serde behavior
    // So we'll handle it explicitly
    let json_missing_value = json!({});
    let result = serde_json::from_value::<TestStruct>(json_missing_value);
    assert!(result.is_err());
}

#[test]
fn test_convert_json_to_yaml() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for test files
    let temp_dir = tempdir()?;
    
    // Create a test JSON file with sample data
    let json_path = temp_dir.path().join("test_affiliations.json");
    let mut json_file = File::create(&json_path)?;
    
    // Write structured test data to the JSON file
    write!(json_file, r#"[
        {{
            "id": "https://ror.org/00aaa1234",
            "name": "Test University",
            "labels": [
                {{ "iso639": "fr", "label": "Université de Test" }},
                {{ "iso639": "de", "label": "Test Universität" }}
            ],
            "acronyms": ["TU", "TEST"]
        }},
        {{
            "id": "https://ror.org/00bbb5678",
            "name": "Another Institute",
            "labels": [],
            "acronyms": []
        }},
        {{
            "id": null,
            "name": "Institute with Null Values",
            "labels": [
                {{ "iso639": null, "label": "Some Label" }},
                {{ "iso639": "es", "label": null }}
            ],
            "acronyms": []
        }}
    ]"#)?;
    json_file.flush()?;
    
    // Path for the output YAML
    let yaml_path = temp_dir.path().join("test_output.yaml");
    
    // Run the conversion
    affiliations::convert_json_to_yaml(&json_path, &yaml_path)?;
    
    // Verify the YAML file was created
    assert!(yaml_path.exists());
    
    // Read the YAML content but skip the BOM at the beginning
    let yaml_content = fs::read_to_string(&yaml_path)?;
    let yaml_content = if yaml_content.starts_with('\u{FEFF}') {
        &yaml_content[3..]  // Skip the BOM
    } else {
        &yaml_content
    };
    
    // Parse the YAML content
    let yaml_data: Vec<affiliations::YamlEntry> = serde_yaml::from_str(yaml_content)?;
    
    // Verify the content
    assert_eq!(yaml_data.len(), 3);
    
    // Check the first entry
    assert_eq!(yaml_data[0].id, "00aaa1234");
    assert_eq!(yaml_data[0].name, "Test University");
    assert_eq!(yaml_data[0].title.get("fr"), Some(&"Universite de Test".to_string()));
    assert_eq!(yaml_data[0].title.get("de"), Some(&"Test Universitat".to_string()));
    assert_eq!(yaml_data[0].acronym, Some("TU".to_string()));
    
    // Check the second entry
    assert_eq!(yaml_data[1].id, "00bbb5678");
    assert_eq!(yaml_data[1].name, "Another Institute");
    assert_eq!(yaml_data[1].acronym, None);
    
    // Check the third entry with null values
    assert_eq!(yaml_data[2].id, "");
    assert_eq!(yaml_data[2].name, "Institute with Null Values");
    assert_eq!(yaml_data[2].title.get("en"), Some(&"Institute with Null Values".to_string()));
    
    Ok(())
}

#[test]
fn test_affiliation_item_deserialization() {
    // Test the AffiliationItem struct deserialization with various input formats
    let json = r#"{
        "id": "https://ror.org/12345",
        "name": "Research Center",
        "labels": [
            {"iso639": "fr", "label": "Centre de Recherche"},
            {"iso639": "es", "label": "Centro de Investigación"}
        ],
        "acronyms": ["RC", "RESC"]
    }"#;
    
    let item: affiliations::AffiliationItem = serde_json::from_str(json).unwrap();
    assert_eq!(item.id, "https://ror.org/12345");
    assert_eq!(item.name, "Research Center");
    assert_eq!(item.labels.len(), 2);
    assert_eq!(item.labels[0].iso639, "fr");
    assert_eq!(item.labels[0].label, "Centre de Recherche");
    assert_eq!(item.acronyms.len(), 2);
    assert_eq!(item.acronyms[0], "RC");
}

#[test]
fn test_edge_cases() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for test files
    let temp_dir = tempdir()?;
    
    // Test with an empty array
    let empty_json_path = temp_dir.path().join("empty.json");
    let mut empty_json_file = File::create(&empty_json_path)?;
    write!(empty_json_file, "[]")?;
    empty_json_file.flush()?;
    
    let empty_yaml_path = temp_dir.path().join("empty_output.yaml");
    affiliations::convert_json_to_yaml(&empty_json_path, &empty_yaml_path)?;
    
    // Read the YAML content but skip the BOM at the beginning
    let empty_yaml_content = fs::read_to_string(&empty_yaml_path)?;
    let empty_yaml_content = if empty_yaml_content.starts_with('\u{FEFF}') {
        &empty_yaml_content[3..]  // Skip the BOM
    } else {
        &empty_yaml_content
    };
    
    let empty_yaml_data: Vec<affiliations::YamlEntry> = serde_yaml::from_str(empty_yaml_content)?;
    assert_eq!(empty_yaml_data.len(), 0);
    
    // Test with special characters in identifiers
    let special_json_path = temp_dir.path().join("special.json");
    let mut special_json_file = File::create(&special_json_path)?;
    write!(special_json_file, r#"[
        {{
            "id": "https://ror.org/special/chars!@#$%",
            "name": "Special Characters Institute",
            "labels": [],
            "acronyms": ["SCI"]
        }}
    ]"#)?;
    special_json_file.flush()?;
    
    let special_yaml_path = temp_dir.path().join("special_output.yaml");
    affiliations::convert_json_to_yaml(&special_json_path, &special_yaml_path)?;
    
    // Read the YAML content but skip the BOM at the beginning
    let special_yaml_content = fs::read_to_string(&special_yaml_path)?;
    let special_yaml_content = if special_yaml_content.starts_with('\u{FEFF}') {
        &special_yaml_content[3..]  // Skip the BOM
    } else {
        &special_yaml_content
    };
    
    let special_yaml_data: Vec<affiliations::YamlEntry> = serde_yaml::from_str(special_yaml_content)?;
    assert_eq!(special_yaml_data.len(), 1);
    assert_eq!(special_yaml_data[0].acronym, Some("SCI".to_string()));
    
    Ok(())
}