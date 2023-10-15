use std::fmt::Display;

use serde::Deserialize;

pub fn get_configurations<'a, T: Deserialize<'a>>() -> Result<T, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to get current dir");
    let config_dir = base_path.join("configurations");
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".into())
        .try_into()
        .expect("Failed to parse the environment");
    let environment_config_filename = format!("{environment}.yaml");
    let configurations = config::Config::builder()
        .add_source(config::File::from(config_dir.join("default.yaml")))
        .add_source(config::File::from(
            config_dir.join(environment_config_filename),
        ))
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    configurations.try_deserialize::<T>()
}

#[derive(Debug)]
pub enum Environment {
    Development,
    Production,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Development => write!(f, "development"),
            Self::Production => write!(f, "production"),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            other => Err(anyhow::anyhow!(
                "{} is not a supported environment. Use either `development` or `production`.",
                other
            )),
        }
    }
}
