use std::{collections::HashMap, path::PathBuf};
use serde::Deserialize;

#[derive(Debug)]
pub struct Duration(std::time::Duration);

impl Duration {
    pub fn as_std(&self) -> std::time::Duration {
        self.0
    }

    pub fn as_chrono(&self) -> chrono::Duration {
        chrono::Duration::from_std(self.0).unwrap()
    }

}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let duration = iso8601::Duration::deserialize(deserializer)?;

        Ok(Duration(std::time::Duration::from(duration)))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StartupOptions {
    Hot,
    Cold,
    #[serde(untagged)]
    Delayed(Duration),
}

#[derive(Debug, Deserialize)]
pub struct RandomRange<T> {
    pub min: T,
    pub max: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrequencyOptions {
    Once,
    #[serde(untagged)]
    Fixed(Duration),
    #[serde(untagged)]
    Random(RandomRange<Duration>),
}

#[derive(Debug, Deserialize)]
pub struct PluginConfig {
    pub name: String,
    pub startup: StartupOptions,
    pub frequency: FrequencyOptions,
    pub healthcheck: Option<String>,
    pub options: Option<HashMap<String, String>>
}

#[derive(Debug, Deserialize)]
pub struct KittypawsConfig {
    pub duration: Option<Duration>,
    pub plugins: Vec<PluginConfig>,
}

impl From<String> for KittypawsConfig {
    fn from(value: String) -> Self {
        serde_yaml::from_str(value.as_str()).unwrap()
    }
}

pub fn load_config(path: PathBuf) -> KittypawsConfig {
    let contents = std::fs::read_to_string(path).expect("Should have been able to read the file");

    KittypawsConfig::from(contents)
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::load_config;

    #[test]
    fn test_correct_configs_loading() {
        load_config(PathBuf::from_str("../configs/dumb_test.yml").unwrap());
    }
}
