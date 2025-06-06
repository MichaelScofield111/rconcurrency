use anyhow::Result;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metric_name: &[&'static str]) -> Self {
        let map = metric_name
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();

        AmapMetrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let count = self
            .data
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("key not fount {}", key))?;

        count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        AmapMetrics {
            data: Arc::clone(&self.data),
        }
    }
}

impl std::fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, count) in self.data.iter() {
            writeln!(f, "{} {}", key, count.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}
