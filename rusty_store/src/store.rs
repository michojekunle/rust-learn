use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    path: PathBuf,
    data: HashMap<String, String>,
}

impl Store {
    /// Create a new Store, loading from path if it exists.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.into();
        let data = if path.exists() {
            let f = File::open(&path)?;
            let reader = BufReader::new(f);
            serde_json::from_reader(reader)?
        } else {
            HashMap::new()
        };

        Ok(Self { path, data })
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Returns true if something was removed
    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    /// Persist the store to disk (overwrites file).
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let f = File::create(&self.path)?;
        let writer = BufWriter::new(f);
        serde_json::to_writer_pretty(writer, &self.data)?;
        Ok(())
    }
}
