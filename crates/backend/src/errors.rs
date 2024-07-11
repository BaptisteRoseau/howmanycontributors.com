use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ConfigParsingError {
    #[error("Error while reading config file")]
    Disconnect(#[from] std::io::Error),

    #[error("{}", .0)]
    Error(String),

    #[error("Config has an invalid YAML format")]
    Parsing(#[from] serde_yaml::Error),
}
