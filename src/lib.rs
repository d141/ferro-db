use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use flate2::{write::GzEncoder, Compression};
use flate2::read::GzDecoder;
use std::io::{Write, Read};
use serde_json;
use std::io;

#[derive(Serialize, Deserialize)]
pub struct FerroDB {
    pub collections: HashMap<String, HashMap<String, String>>,
    pub active_collection: Option<String>,
    changes: Vec<Change>,
}

#[derive(Serialize, Deserialize, Clone)]
enum Operation {
    Create { key: String, value: String },
    Update { key: String, old_value: String, new_value: String },
    Delete { key: String, value: String },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Change {
    collection: String,
    operation: Operation,
}

fn save_changes_to_file(changes: &[Change], filename: &str) -> io::Result<()> {
    let serialized = serde_json::to_string(changes)?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(serialized.as_bytes())?;
    let compressed_data = encoder.finish()?;
    std::fs::write(filename, compressed_data)
}


impl FerroDB {
    pub fn new() -> FerroDB {
        FerroDB {
            collections: HashMap::new(),
            active_collection: None,
            changes: Vec::new(),
        }
    }

    pub fn create_collection(&mut self, name: String) {
        self.collections.insert(name, HashMap::new());
    }

    pub fn use_collection(&mut self, name: String) -> Result<(), String> {
        if self.collections.contains_key(&name) {
            self.active_collection = Some(name);
            Ok(())
        } else {
            Err("Collection does not exist".to_string())
        }
    }
    pub fn set(&mut self, collection: &str, key: String, value: String) -> Result<(), String> {
        let coll = self.collections.entry(collection.to_string()).or_insert_with(HashMap::new);
        if let Some(old_value) = coll.get(&key) {
            // Log the change as an update
            self.changes.push(Change {
                collection: collection.to_string(),
                operation: Operation::Update {
                    key: key.clone(),
                    old_value: old_value.clone(),
                    new_value: value.clone(),
                },
            });
            coll.insert(key, value);  // Update the existing key with the new value
        } else {
            // Log the change as a create
            self.changes.push(Change {
                collection: collection.to_string(),
                operation: Operation::Create {
                    key: key.clone(),
                    value: value.clone(),
                },
            });
            coll.insert(key, value);  // Insert the new key-value pair
        }
        Ok(())
    }

    // Get the value of a key in a specified collection
    pub fn get(&self, collection: &str, key: &str) -> Result<Option<String>, String> {
        if let Some(coll) = self.collections.get(collection) {
            Ok(coll.get(key).cloned())
        } else {
            Err("Collection not found".to_string())
        }
    }

    // Unset (delete) a key-value pair in the specified collection
    pub fn unset(&mut self, collection: &str, key: &str) -> Result<Option<String>, String> {
        if let Some(coll) = self.collections.get_mut(collection) {
            if let Some(value) = coll.remove(key) {
                // Log the change as a delete
                self.changes.push(Change {
                    collection: collection.to_string(),
                    operation: Operation::Delete {
                        key: key.to_string(),
                        value: value.clone(),
                    },
                });
                Ok(Some(value))
            } else {
                Ok(None)  // Key not found
            }
        } else {
            Err("Collection not found".to_string())
        }
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string(&self.collections)?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(serialized.as_bytes())?;
        let compressed_data = encoder.finish()?;
        std::fs::write(filename, compressed_data)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self, std::io::Error> {
        let compressed_data = std::fs::read(filename)?;
        let mut decoder = GzDecoder::new(&compressed_data[..]);
        let mut decompressed_data = String::new();
        decoder.read_to_string(&mut decompressed_data)?;
        let collections: HashMap<String, HashMap<String, String>> = serde_json::from_str(&decompressed_data)?;
        Ok(FerroDB {
            collections,
            active_collection: None,
            changes: Vec::new(),
        })
    }

    pub fn save_changes(&self) -> Result<(), std::io::Error> {
        let filename = "changes.fdbz"; // Using a custom file extension
        save_changes_to_file(&self.changes, filename)
    }

    pub fn apply_change(&mut self, change: &Change) {
        let collection = self.collections.get_mut(&change.collection).unwrap();
        match &change.operation {
            Operation::Create { key, value } => { collection.insert(key.clone(), value.clone()); },
            Operation::Update { key, new_value, .. } => { collection.insert(key.clone(), new_value.clone()); },
            Operation::Delete { key, .. } => { collection.remove(key); },
        }
    }

    pub fn revert_change(&mut self, change: &Change) {
        let collection = self.collections.get_mut(&change.collection).unwrap();
        match &change.operation {
            Operation::Create { key, .. } => { collection.remove(key); },
            Operation::Update { key, old_value, .. } => { collection.insert(key.clone(), old_value.clone()); },
            Operation::Delete { key, value } => { collection.insert(key.clone(), value.clone()); },
        }
    }

}