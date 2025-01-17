use crate::config::{Config, MappingType, TableMapping};
use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::fs;

pub struct Parser {
    config: Config,
    output_dir: PathBuf,
    tables: HashMap<String, TableData>,
}

#[derive(Debug)]
struct TableData {
    headers: Vec<String>,
    rows: Vec<HashMap<String, String>>,
    primary_keys: HashSet<String>,
}

impl TableData {
    fn new(headers: Vec<String>) -> Self {
        TableData {
            headers,
            rows: Vec::new(),
            primary_keys: HashSet::new(),
        }
    }
}

impl Parser {
    pub fn new(config: Config, output_dir: PathBuf) -> Self {
        Self {
            config,
            output_dir,
            tables: HashMap::new(),
        }
    }

    pub fn process_file(&mut self, input_path: &Path) -> Result<()> {
        let file_content = std::fs::read_to_string(input_path)
            .with_context(|| format!("Failed to read file: {}", input_path.display()))?;
        
        let json: Value = serde_json::from_str(&file_content)
            .with_context(|| format!("Failed to parse JSON from file: {}", input_path.display()))?;

        let root_value = if self.config.parameters.root_node.is_empty() {
            json
        } else {
            self.get_root_node(&json, &self.config.parameters.root_node)?.clone()
        };

        let file_name = input_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        if self.config.parameters.add_file_name {
            let root_table = self.tables.entry("root".to_string()).or_insert_with(|| TableData {
                headers: vec!["keboola_file_name_col".to_string()],
                rows: Vec::new(),
                primary_keys: HashSet::new(),
            });
            if !root_table.headers.contains(&"keboola_file_name_col".to_string()) {
                root_table.headers.push("keboola_file_name_col".to_string());
            }
        }

        self.process_value(&root_value, "root".to_string(), None, &file_name)?;
        Ok(())
    }

    fn process_value(&mut self, value: &Value, table_name: String, path: Option<String>, file_name: &str) -> Result<()> {
        match value {
            Value::Object(obj) => {
                let mut row = HashMap::new();
                let mut table_updates = Vec::new();
                let mut table_mappings = Vec::new();

                for (key, val) in obj {
                    if let Some(mapping_type) = self.config.parameters.mapping.get(key).cloned() {
                        match mapping_type {
                            MappingType::Column { mapping } => {
                                row.insert(mapping.destination.clone(), self.format_value(val));
                            }
                            MappingType::Table(table_mapping) => {
                                table_mappings.push((val, table_mapping));
                            }
                        }
                    } else {
                        match val {
                            Value::Object(_) => {
                                continue;
                            }
                            Value::Array(arr) => {
                                let mut items = Vec::new();
                                for (i, item) in arr.iter().enumerate() {
                                    let parent_id = format!("{}_{}", key, i);
                                    if let Value::Object(item_obj) = item {
                                        let mut item_row = HashMap::new();
                                        for (item_key, item_val) in item_obj {
                                            item_row.insert(format!("item_{}", item_key), self.format_value(item_val));
                                        }
                                        item_row.insert("JSON_parentId".to_string(), parent_id);
                                        items.push(item_row);
                                    }
                                }
                                if !items.is_empty() {
                                    table_updates.push((key.clone(), items));
                                }
                            }
                            _ => {
                                row.insert(key.clone(), self.format_value(val));
                            }
                        }
                    }
                }

                if self.config.parameters.add_file_name && (path.is_none() || path.as_ref().unwrap() == "root") {
                    row.insert("keboola_file_name_col".to_string(), format!("{} ", file_name));
                }

                let mut table = self.tables.entry(table_name.clone()).or_insert_with(|| {
                    let mut headers = Vec::new();
                    if self.config.parameters.add_file_name && (path.is_none() || path.as_ref().unwrap() == "root") {
                        headers.push("id".to_string());
                        headers.push("name".to_string());
                        headers.push("keboola_file_name_col".to_string());
                    }
                    TableData {
                        headers,
                        rows: Vec::new(),
                        primary_keys: HashSet::new(),
                    }
                });
                table.rows.push(row);

                for (key, items) in table_updates {
                    let mut item_table = self.tables.entry(key).or_insert_with(|| {
                        TableData {
                            headers: vec!["item_id".to_string(), "quantity".to_string(), "JSON_parentId".to_string()],
                            rows: Vec::new(),
                            primary_keys: HashSet::new(),
                        }
                    });
                    item_table.rows.extend(items);
                }

                for (val, table_mapping) in table_mappings {
                    self.process_table_mapping(val, &table_mapping, file_name)?;
                }
            }
            Value::Array(arr) => {
                for item in arr {
                    self.process_value(item, table_name.clone(), path.clone(), file_name)?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn process_table_mapping(&mut self, value: &Value, mapping: &TableMapping, file_name: &str) -> Result<()> {
        let root_value = if !self.config.parameters.root_node.is_empty() {
            self.get_root_node(value, &self.config.parameters.root_node)?
        } else {
            value
        };

        match root_value {
            Value::Array(arr) => {
                for order in arr {
                    if let Some(obj) = order.as_object() {
                        if let Some(id) = obj.get("id") {
                            if let Some(items) = obj.get("items") {
                                if let Some(items_arr) = items.as_array() {
                                    for item in items_arr {
                                        let mut row = HashMap::new();
                                        row.insert("order_id".to_string(), self.format_value(id));
                                        if let Some(item_obj) = item.as_object() {
                                            for (key, val) in item_obj {
                                                row.insert(key.clone(), self.format_value(val));
                                            }
                                        }
                                        let table_name = mapping.destination.clone();
                                        let mut table = self.tables.entry(table_name).or_insert_with(|| {
                                            let mut headers = Vec::new();
                                            headers.push("order_id".to_string());
                                            headers.push("item_id".to_string());
                                            headers.push("quantity".to_string());
                                            TableData {
                                                headers,
                                                rows: Vec::new(),
                                                primary_keys: HashSet::new(),
                                            }
                                        });
                                        table.rows.push(row);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Value::Object(obj) => {
                if let Some(id) = obj.get("id") {
                    if let Some(items) = obj.get("items") {
                        if let Some(items_arr) = items.as_array() {
                            for item in items_arr {
                                let mut row = HashMap::new();
                                row.insert("order_id".to_string(), self.format_value(id));
                                if let Some(item_obj) = item.as_object() {
                                    for (key, val) in item_obj {
                                        row.insert(key.clone(), self.format_value(val));
                                    }
                                }
                                let table_name = mapping.destination.clone();
                                let mut table = self.tables.entry(table_name).or_insert_with(|| {
                                    let mut headers = Vec::new();
                                    headers.push("order_id".to_string());
                                    headers.push("item_id".to_string());
                                    headers.push("quantity".to_string());
                                    TableData {
                                        headers,
                                        rows: Vec::new(),
                                        primary_keys: HashSet::new(),
                                    }
                                });
                                table.rows.push(row);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_root_node<'a>(&self, json: &'a Value, root_node: &str) -> Result<&'a Value> {
        let mut current = json;
        if !root_node.is_empty() {
            for node in root_node.split('.') {
                current = current.get(node).ok_or_else(|| {
                    anyhow::anyhow!("Root node path '{}' not found in JSON", root_node)
                })?;
            }
        }
        Ok(current)
    }

    fn format_value(&self, value: &Value) -> String {
        match value {
            Value::String(s) => format!("{}", s),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => String::new(),
            _ => value.to_string().trim_matches('"').to_string()
        }
    }

    pub fn write_tables(&self) -> Result<()> {
        for (table_name, data) in &self.tables {
            let output_path = self.output_dir.join(format!("{}.csv", table_name));
            // Ensure output directory exists
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut writer = csv::WriterBuilder::new()
                .quote_style(csv::QuoteStyle::Always)
                .from_path(&output_path)?;

            // Write headers
            writer.write_record(&data.headers)?;

            // Write rows
            for row in &data.rows {
                let record: Vec<_> = data.headers.iter()
                    .map(|header| row.get(header).map(String::as_str).unwrap_or(""))
                    .collect();
                writer.write_record(&record)?;
            }

            writer.flush()?;
        }
        Ok(())
    }
} 