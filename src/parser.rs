use crate::config::{Config, MappingType, TableMapping};
use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub struct Parser {
    config: Config,
    output_dir: PathBuf,
}

#[derive(Debug)]
struct TableData {
    headers: Vec<String>,
    rows: Vec<HashMap<String, String>>,
    primary_keys: HashSet<String>,
}

impl Parser {
    pub fn new(config: Config, output_dir: PathBuf) -> Self {
        Self { config, output_dir }
    }

    pub fn process_file(&self, input_path: &Path) -> Result<()> {
        let file_content = std::fs::read_to_string(input_path)
            .with_context(|| format!("Failed to read file: {}", input_path.display()))?;
        
        let json: Value = serde_json::from_str(&file_content)
            .with_context(|| format!("Failed to parse JSON from file: {}", input_path.display()))?;

        let root_value = if self.config.parameters.root_node.is_empty() {
            json
        } else {
            self.get_root_node(&json)?.clone()
        };

        let mut tables = HashMap::new();
        self.process_value(&root_value, "root".to_string(), &mut tables, None)?;

        for (table_name, table_data) in tables {
            self.write_csv_table(&table_name, table_data)?;
        }

        Ok(())
    }

    fn get_root_node<'a>(&self, json: &'a Value) -> Result<&'a Value> {
        let mut current = json;
        for key in self.config.parameters.root_node.split('.') {
            current = current.get(key)
                .with_context(|| format!("Root node path '{}' not found in JSON", key))?;
        }
        Ok(current)
    }

    fn process_value(
        &self,
        value: &Value,
        path: String,
        tables: &mut HashMap<String, TableData>,
        parent_id: Option<String>,
    ) -> Result<()> {
        match value {
            Value::Object(obj) => {
                let mut row = HashMap::new();
                
                if self.config.parameters.add_file_name {
                    row.insert("keboola_file_name_col".to_string(), "".to_string());
                }

                for (key, val) in obj {
                    let new_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };

                    match self.config.parameters.mapping.get(&new_path) {
                        Some(MappingType::Column { mapping }) => {
                            row.insert(mapping.destination.clone(), val.to_string());
                        }
                        Some(MappingType::Table(table_mapping)) => {
                            self.process_table_mapping(val, table_mapping, tables)?;
                        }
                        None => {
                            row.insert(key.clone(), val.to_string());
                        }
                    }
                }

                if let Some(parent_id) = parent_id {
                    row.insert("JSON_parentId".to_string(), parent_id);
                }

                let table_name = if path.is_empty() { "root".to_string() } else { path };
                let table = tables.entry(table_name).or_insert_with(|| TableData {
                    headers: row.keys().cloned().collect(),
                    rows: Vec::new(),
                    primary_keys: HashSet::new(),
                });
                table.rows.push(row);
            }
            Value::Array(arr) => {
                for (i, item) in arr.iter().enumerate() {
                    let item_id = format!("{}_{}", path, i);
                    self.process_value(item, path.clone(), tables, Some(item_id))?;
                }
            }
            _ => {
                // Handle primitive values if needed
            }
        }
        Ok(())
    }

    fn process_table_mapping(
        &self,
        value: &Value,
        mapping: &TableMapping,
        tables: &mut HashMap<String, TableData>,
    ) -> Result<()> {
        let mut table = tables
            .entry(mapping.destination.clone())
            .or_insert_with(|| TableData {
                headers: Vec::new(),
                rows: Vec::new(),
                primary_keys: HashSet::new(),
            });

        match value {
            Value::Object(obj) => {
                let mut row = HashMap::new();
                for (key, val) in obj {
                    if let Some(field_mapping) = mapping.table_mapping.get(key) {
                        match field_mapping {
                            MappingType::Column { mapping } => {
                                row.insert(mapping.destination.clone(), val.to_string());
                                if mapping.primary_key {
                                    table.primary_keys.insert(mapping.destination.clone());
                                }
                            }
                            MappingType::Table(_) => {
                                // Nested tables not supported in this implementation
                            }
                        }
                    }
                }
                if table.headers.is_empty() {
                    table.headers = row.keys().cloned().collect();
                }
                table.rows.push(row);
            }
            Value::Array(arr) => {
                for item in arr {
                    if let Value::Object(_) = item {
                        self.process_table_mapping(item, mapping, tables)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn write_csv_table(&self, table_name: &str, data: TableData) -> Result<()> {
        let output_path = self.output_dir.join(format!("{}.csv", table_name));
        let mut writer = csv::Writer::from_path(&output_path)?;

        // Write headers
        writer.write_record(&data.headers)?;

        // Write rows
        for row in data.rows {
            let record: Vec<String> = data.headers
                .iter()
                .map(|header| row.get(header).cloned().unwrap_or_default())
                .collect();
            writer.write_record(&record)?;
        }

        writer.flush()?;
        Ok(())
    }
} 