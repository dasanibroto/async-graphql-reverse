use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use toml;

#[derive(Deserialize, Debug)]
pub struct ResolverSetting {
    pub target_type: String,
    pub target_field: String,
    pub resolver_type: String,
}

#[derive(Deserialize, Debug)]
pub struct RendererConfig {
    pub using: HashMap<String, String>,
    pub resolver: Option<Vec<ResolverSetting>>,
}

impl RendererConfig {
    pub fn data_source_using(&self) -> String {
        self.using
            .get("data_source")
            .map(|v| v.to_string())
            .unwrap_or_else(|| "use crate::datasource::DataSource".to_string())
    }

    pub fn resolver_setting(&self) -> HashMap<String, HashMap<String, String>> {
        match self.resolver.as_ref() {
            None => return HashMap::new(),
            Some(resolver) => {
                if resolver.is_empty() {
                    return HashMap::new();
                } else {
                    let mut result = HashMap::<String, HashMap<String, String>>::new();
                    for each_resolver in resolver.iter() {
                        let field_and_resolver_type = result
                            .entry(each_resolver.target_type.to_string())
                            .or_insert(HashMap::<String, String>::new());
                        field_and_resolver_type.insert(
                            each_resolver.target_field.to_string(),
                            each_resolver.resolver_type.to_string(),
                        );
                    }
                    result
                }
            }
        }
    }

    pub fn load(file_path: &str) -> Result<RendererConfig> {
        let toml_str: String = fs::read_to_string(file_path)?;
        let config: RendererConfig = toml::from_str(&toml_str).map_err(|e| anyhow!("{}", e))?;
        Ok(config)
    }
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            using: HashMap::new(),
            resolver: None,
        }
    }
}
