// metrics data structure
// basement: inc/dec/snapshot
use anyhow::Result;
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        data.entry(key.into())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .read()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .clone())
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.data.read().map_err(|_e| fmt::Error {})?;
        for (key, count) in data.iter() {
            writeln!(f, "{} {}", key, count)?;
        }
        Ok(())
    }
}
