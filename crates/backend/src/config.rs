use crate::errors::ConfigParsingError;
use clap::Parser;
use log::warn;
use serde::Deserialize;
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};

const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const DEFAULT_PORT: u16 = 6969;

const DEFAULT_PROMETHEUS_IP: IpAddr = LOCALHOST;
const DEFAULT_PROMETHEUS_PORT: u16 = 9090;

const DEFAULT_CACHE_URLS: &str = "redis://hmc-redis:6379/";
const DEFAULT_CACHE_USER: &str = "backend";
const DEFAULT_CACHE_PASSWORD: &str = "password";
const DEFAULT_CACHE_VALIDITY_SEC_MIN: usize = 259200; // 3 days
const DEFAULT_CACHE_VALIDITY_SEC_MAX: usize = 345600; // 4 days

const DEFAULT_CONFIG_FILE_PATH: &str = ".config.yaml";

/* ======================================================================================
FULL CONFIG FROM USER
====================================================================================== */

///

/// hmc backend server configuration.
///
/// This struct serves as a parser for the configuration file and command line arguments.
/// It is then parsed to build the full configuration [`Config`] for the server.
///
/// This is done like this to keep all arguments available via the configuration file or
/// CLI, while allowing the [`Config`]'s substructures to be valid for the rest of the program.
/// For example, the [`PrometheusConfig`] and [`PostgresConfig`] will be built if and only
/// if all their parameters are provided, hence no need to check each of them in the client code.
///
/// CLI arguments grouped together into a single struct should be prefixed with the same
/// name.
/// For example, all arguments related to the CACHE should be prefixed with `CACHE_`.
#[derive(Parser, Deserialize, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct CliConfig {
    /// Path to the configuration file
    #[arg(short, long, env)]
    pub(crate) config: Option<PathBuf>,

    /// The IP where to bind the server
    #[arg(short, long, env, default_value_t = LOCALHOST)]
    pub(crate) ip: IpAddr,

    /// The port where to bind the server
    #[arg(short, long, env, default_value_t = DEFAULT_PORT)]
    pub(crate) port: u16,

    /// The PEM private key file used for HTTPS and JWT.
    /// If not provided, default to HTTP and static token for JWT.
    /// Required in production.
    #[arg(long, env)]
    pub(crate) pem_priv_key: Option<PathBuf>,

    /// The PEM public key file used for HTTPS and JWT
    /// If not provided, default to HTTP and static token for JWT.
    /// Required in production.
    #[arg(long, env)]
    pub(crate) pem_pub_key: Option<PathBuf>,

    /* ===============
    CACHE
    ================ */
    /// CACHE urls. ','-separated list of URLs to connect to.
    /// Example: redis://redis-srv1:6379/,redis://user:password@redis-srv1:6379/
    #[arg(long, env, default_value_t = DEFAULT_CACHE_URLS.to_string())]
    pub(crate) cache_cluster_urls: String,

    /// CACHE user
    #[arg(long, env, default_value_t = DEFAULT_CACHE_USER.to_string())]
    pub(crate) cache_user: String,

    /// CACHE password
    #[arg(long, env, default_value_t = DEFAULT_CACHE_PASSWORD.to_string())]
    pub(crate) cache_password: String,

    /// CACHE minimum Time To Live in seconds
    #[arg(long, env, default_value_t = DEFAULT_CACHE_VALIDITY_SEC_MIN)]
    pub(crate) cache_ttl_sec_min: usize,

    /// CACHE maximum Time To Live in seconds
    #[arg(long, env, default_value_t = DEFAULT_CACHE_VALIDITY_SEC_MAX)]
    pub(crate) cache_ttl_sec_max: usize,

    /* ===============
    PROMETHEUS
    ================ */
    /// Prometheus server host
    #[arg(long, env, default_value_t = DEFAULT_PROMETHEUS_IP)]
    pub(crate) prometheus_ip: IpAddr,

    /// Prometheus server port
    #[arg(long, env, default_value_t = DEFAULT_PROMETHEUS_PORT)]
    pub(crate) prometheus_port: u16,

    /// Deactivate Prometheus metric server
    #[arg(long, env, default_value_t = false)]
    pub(crate) no_prometheus: bool,
}

impl CliConfig {
    /// Loads the configuration file and updates its value with the provided CLI/ENV arguments.
    ///
    /// The CLI/ENV arguments take precedence over the configuration file.
    pub(crate) fn parse_with_file() -> Result<CliConfig, ConfigParsingError> {
        let mut config: CliConfig = Self::parse();

        let mut file_config: Option<CliConfig> = None;
        if let Some(file) = &config.config {
            file_config = Some(serde_yaml::from_str(fs::read_to_string(file)?.as_str())?);
        } else if Path::new(DEFAULT_CONFIG_FILE_PATH).is_file() {
            file_config = Some(serde_yaml::from_str(
                fs::read_to_string(DEFAULT_CONFIG_FILE_PATH)?.as_str(),
            )?);
        }

        if let Some(file_config) = file_config {
            config = file_config.merge(config);
        }

        Ok(config)
    }

    /// Overwrites the current configuration with the provided one.
    fn merge(&self, other: CliConfig) -> CliConfig {
        // Add warnings for keys that are being overridden
        let _ = other;
        todo!("Config file not supported yet");
    }

    /// Generates a default configuration file template.
    #[allow(dead_code)]
    pub(crate) fn template() -> String {
        todo!("Use clap to generate a default configuration template with documentation and commented defaults");
    }
}

/* ======================================================================================
CONFIG
====================================================================================== */

#[derive(Debug, Clone)]
pub(crate) struct BindingConfig {
    pub(crate) ip: IpAddr,
    pub(crate) port: u16,
}

#[derive(Debug, Clone)]
pub(crate) struct Cache {
    pub(crate) urls: Vec<String>,
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) ttl_sec_min: usize,
    pub(crate) ttl_sec_max: usize,
}

type ServerBindingConfig = BindingConfig;
type PrometheusConfig = BindingConfig;

#[derive(Debug, Clone)]
pub(crate) struct TlsConfig {
    pub(crate) private_key: PathBuf,
    pub(crate) public_key: PathBuf,
}

/// The main configuration.
///
/// This struct is passed to the whole program to configure the server.
/// All of its attributes are considered valid and should be used as is if not None.
///
/// Any user input validation should be done within this struct,
/// in the [`Config::validate`] method.
#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub(crate) server: ServerBindingConfig,
    pub(crate) cache: Cache,
    pub(crate) pem: Option<TlsConfig>,
    pub(crate) prometheus: Option<PrometheusConfig>,
}

impl Config {
    pub(crate) fn parse() -> Result<Self, ConfigParsingError> {
        Self::try_from(CliConfig::parse_with_file()?)
    }
}

impl TryFrom<CliConfig> for Config {
    type Error = ConfigParsingError;

    fn try_from(value: CliConfig) -> Result<Self, ConfigParsingError> {
        Self::validate(&value)?;

        let prometheus = if value.no_prometheus {
            None
        } else {
            Some(PrometheusConfig {
                ip: value.prometheus_ip,
                port: value.prometheus_port,
            })
        };

        let pem = if value.pem_priv_key.is_some() && value.pem_pub_key.is_some() {
            Some(TlsConfig {
                private_key: value.pem_priv_key.unwrap(),
                public_key: value.pem_pub_key.unwrap(),
            })
        } else {
            None
        };

        Ok(Self {
            server: ServerBindingConfig {
                ip: value.ip,
                port: value.port,
            },
            cache: Cache {
                urls: value
                    .cache_cluster_urls
                    .split(',')
                    .map(String::from)
                    .collect::<Vec<_>>(),
                user: value.cache_user,
                password: value.cache_password,
                ttl_sec_min: value.cache_ttl_sec_min,
                ttl_sec_max: value.cache_ttl_sec_max,
            },
            prometheus,
            pem,
        })
    }
}

impl Config {
    /// Verifies the CLI configuration is valid, throw a [`ConfigParsingError`] is not.
    ///
    /// For example, makes sure the PEM key **AND** certificate are provided
    /// if the server is in production mode.
    fn validate(cli_config: &CliConfig) -> Result<(), ConfigParsingError> {
        // Errors
        if cli_config.pem_priv_key.is_some() && cli_config.pem_pub_key.is_none() {
            return Err(ConfigParsingError::MissingPemPubCert);
        }
        if cli_config.pem_priv_key.is_none() && cli_config.pem_pub_key.is_some() {
            return Err(ConfigParsingError::MissingPemPrivKey);
        }
        #[cfg(not(debug_assertions))]
        if cli_config.password_salt == String::from(DEFAULT_SALT) {
            return Err(ConfigParsingError::DefaultPasswordSaltInReleaseMode);
        }
        if cli_config.cache_ttl_sec_max < cli_config.cache_ttl_sec_min {
            return Err(ConfigParsingError::Error(
                "Cache TTL max must be greater or equal to min".to_string(),
            ));
        }

        if cli_config.no_prometheus
            && (cli_config.prometheus_ip != DEFAULT_PROMETHEUS_IP
                || cli_config.prometheus_port != DEFAULT_PROMETHEUS_PORT)
        {
            warn!("Ignoring Prometheus server configuration because it is deactivated.");
        }

        Ok(())
    }
}

mod test {
    //TODO: Config priority: default->file->env->cli
    //TODO: CliConfig merging priority: self->other
    use super::*;

    impl Default for CliConfig {
        fn default() -> Self {
            CliConfig {
                config: None,
                ip: LOCALHOST,
                port: DEFAULT_PORT,
                pem_priv_key: None,
                pem_pub_key: None,
                cache_cluster_urls: DEFAULT_CACHE_URLS.to_string(),
                cache_user: DEFAULT_CACHE_USER.to_string(),
                cache_password: DEFAULT_CACHE_PASSWORD.to_string(),
                cache_ttl_sec_min: DEFAULT_CACHE_VALIDITY_SEC_MIN,
                cache_ttl_sec_max: DEFAULT_CACHE_VALIDITY_SEC_MAX,
                prometheus_ip: DEFAULT_PROMETHEUS_IP,
                prometheus_port: DEFAULT_PROMETHEUS_PORT,
                no_prometheus: false,
            }
        }
    }

    #[test]
    fn test_validate_missing_pem_pub_cert() {
        let mut cli_config = CliConfig::default();
        cli_config.pem_priv_key = Some(PathBuf::from("private_key.pem"));
        cli_config.pem_pub_key = None;

        let result = Config::validate(&cli_config);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigParsingError::MissingPemPubCert
        ));
    }

    #[test]
    fn test_validate_missing_pem_priv_key() {
        let mut cli_config = CliConfig::default();
        cli_config.pem_pub_key = Some(PathBuf::from("public_key.pem"));
        cli_config.pem_priv_key = None;

        let result = Config::validate(&cli_config);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigParsingError::MissingPemPrivKey
        ));
    }

    #[test]
    fn test_validate_ignore_swagger_config() {
        let cli_config = CliConfig::default();

        let result = Config::validate(&cli_config);
        let config = Config::try_from(cli_config);

        assert!(result.is_ok());
        assert!(config.is_ok());
    }

    #[test]
    fn test_validate_ignore_prometheus_config() {
        let mut cli_config = CliConfig::default();
        cli_config.no_prometheus = true;
        cli_config.prometheus_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        cli_config.prometheus_port = 9091;

        let result = Config::validate(&cli_config);
        let config = Config::try_from(cli_config);

        assert!(result.is_ok());
        assert!(config.is_ok());
        assert!(config.unwrap().prometheus.is_none());
    }
}
