//! Shared config loader for probe binaries.
//!
//! Reads `probe_paths.conf` (gitignored) from the crate root.
//! Each line is either a comment (`#`) or a `key=value` pair.
//! See `probe_paths.conf.example` for the template.

use std::collections::HashMap;

const CONFIG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/probe_paths.conf");

pub struct ProbeConfig {
    map: HashMap<String, String>,
}

impl ProbeConfig {
    pub fn load() -> Self {
        let content = std::fs::read_to_string(CONFIG_PATH).unwrap_or_else(|_| {
            panic!(
                "probe_paths.conf not found at {CONFIG_PATH}\n\
                 Copy probe_paths.conf.example to probe_paths.conf and set your paths."
            )
        });
        let map = content
            .lines()
            .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
            .filter_map(|l| l.split_once('='))
            .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
            .collect();
        ProbeConfig { map }
    }

    /// Returns the value for `key`, panicking with a clear message if absent.
    pub fn get(&self, key: &str) -> &str {
        self.map.get(key).unwrap_or_else(|| {
            panic!("Missing key '{key}' in probe_paths.conf")
        })
    }

    /// Returns all configured scan roots in definition order.
    pub fn scan_roots(&self) -> Vec<&str> {
        ["pcgen_data", "bahamut_data"]
            .iter()
            .filter_map(|k| self.map.get(*k).map(|s| s.as_str()))
            .collect()
    }
}
