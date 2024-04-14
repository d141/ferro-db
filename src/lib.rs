use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use flate2::{write::GzEncoder, Compression};
use flate2::read::GzDecoder;
use std::io::{Write, Read};

#[derive(Serialize, Deserialize)]
pub struct FerroDB {
    pub collections: HashMap<String, HashMap<String, String>>,
    pub active_collection: Option<String>,
}


impl FerroDB {
    pub fn new() -> FerroDB {
        FerroDB {
            collections: HashMap::new(),
            active_collection: None,
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

    pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
        match &self.active_collection {
            Some(collection_name) => {
                if let Some(collection) = self.collections.get_mut(collection_name) {
                    if collection.contains_key(&key) {
                        Err("Error: Key already exists. Please UNSET first.".to_string())
                    } else {
                        collection.insert(key, value);
                        Ok(())
                    }
                } else {
                    Err("Active collection not found.".to_string())
                }
            },
            None => Err("No collection selected. Please use a collection first.".to_string()),
        }
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, String> {
        if let Some(collection_name) = &self.active_collection {
            if let Some(collection) = self.collections.get(collection_name) {
                Ok(collection.get(key).cloned())
            } else {
                Err("Active collection not found.".to_string())
            }
        } else {
            Err("No collection selected. Please use a collection first.".to_string())
        }
    }

    pub fn unset(&mut self, key: &str) -> Result<Option<String>, String> {
        if let Some(collection_name) = &self.active_collection {
            if let Some(collection) = self.collections.get_mut(collection_name) {
                Ok(collection.remove(key))
            } else {
                Err("Active collection not found.".to_string())
            }
        } else {
            Err("No collection selected. Please use a collection first.".to_string())
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
        })
    }

}