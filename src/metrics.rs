// metrics data structure
// basement: inc/dec/snapshot
use anyhow::Result;
use std::{fmt, sync::Arc};

use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        self.data
            .entry(key.into())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        Ok(())
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}, {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
