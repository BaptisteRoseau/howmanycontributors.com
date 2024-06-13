use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ConfigParsingError {
    #[error("Error while reading config file")]
    Disconnect(#[from] std::io::Error),

    #[error("Missing PEM public certificate")]
    MissingPemPubCert,

    #[error("Missing PEM private key")]
    MissingPemPrivKey,

    #[error("Config has an invalid YAML format")]
    Parsing(#[from] serde_yaml::Error),
}
