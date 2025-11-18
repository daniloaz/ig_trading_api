use reqwest::header::HeaderMap;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::str::FromStr;
use std::string::ToString;

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// COMMON TYPES FOR BOTH THE REST AND STREAMING APIS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Enum to represent the execution environment (DEMO or LIVE).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ExecutionEnvironment {
    Demo,
    Live,
}

impl Default for ExecutionEnvironment {
    fn default() -> Self {
        ExecutionEnvironment::Demo
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum LogType
{
    StdLogs,
    TracingLogs,
}

impl<'de> Deserialize<'de> for ExecutionEnvironment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_uppercase();

        match s.as_str() {
            "DEMO" => Ok(ExecutionEnvironment::Demo),
            "LIVE" => Ok(ExecutionEnvironment::Live),
            _ => Err(serde::de::Error::custom("Invalid account type")),
        }
    }
}

impl FromStr for ExecutionEnvironment {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DEMO" => Ok(ExecutionEnvironment::Demo),
            "LIVE" => Ok(ExecutionEnvironment::Live),
            _ => Err(ApiError {
                message: format!("Invalid account type: {}", s),
            }),
        }
    }
}

/// Struct to hold IG API configuration data, both for the REST and streaming APIs.
/// 
/// Sensitive data (credentials, API keys, account numbers) are loaded from environment variables,
/// while non-sensitive application behavior settings are loaded from config.yaml.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ApiConfig {
    /// Your demo IG account number (loaded from IG_ACCOUNT_NUMBER_DEMO env var).
    #[serde(skip_deserializing)]
    pub account_number_demo: String,
    /// Your live IG account number (loaded from IG_ACCOUNT_NUMBER_LIVE env var).
    #[serde(skip_deserializing)]
    pub account_number_live: String,
    /// An alternative account number for testing (loaded from IG_ACCOUNT_NUMBER_TEST env var).
    /// It must be associated to the same running environment as the main account number.
    #[serde(skip_deserializing)]
    pub account_number_test: Option<String>,
    /// The API key assigned to your IG account (loaded from IG_API_KEY env var).
    #[serde(skip_deserializing)]
    pub api_key: String,
    /// Automatically log in to the API on instantiation and when the session expires.
    pub auto_login: Option<bool>,
    /// The base URL for the demo environment (loaded from IG_BASE_URL_DEMO env var).
    #[serde(skip_deserializing)]
    pub base_url_demo: String,
    /// The base URL for the live environment (loaded from IG_BASE_URL_LIVE env var).
    #[serde(skip_deserializing)]
    pub base_url_live: String,
    /// The execution environment (loaded from IG_EXECUTION_ENVIRONMENT env var: DEMO or LIVE).
    #[serde(skip_deserializing)]
    pub execution_environment: ExecutionEnvironment,
    /// Logging mechanism
    pub logger: LogType,
    /// Your user password (loaded from IG_PASSWORD env var).
    #[serde(skip_deserializing)]
    pub password: String,
    /// The session version to use for login requests.
    pub session_version: Option<usize>,
    /// The maximum number of connection attempts for the streaming API.
    pub streaming_api_max_connection_attempts: Option<u64>,
    /// Your username (loaded from IG_USERNAME env var).
    #[serde(skip_deserializing)]
    pub username: String,
}

impl ApiConfig {
    /// Create a new ApiConfig instance with empty values.
    pub fn new() -> Self {
        ApiConfig {
            account_number_demo: "".to_string(),
            account_number_live: "".to_string(),
            account_number_test: None,
            api_key: "".to_string(),
            auto_login: None,
            base_url_demo: "".to_string(),
            base_url_live: "".to_string(),
            execution_environment: ExecutionEnvironment::Demo,
            logger: LogType::StdLogs,
            password: "".to_string(),
            session_version: None,
            streaming_api_max_connection_attempts: None,
            username: "".to_string(),
        }
    }

    /// Load environment variables from .env file and environment.
    /// This should be called before accessing any ApiConfig instance.
    pub fn load_env() -> Result<(), Box<dyn Error>> {
        // Load .env file if it exists (optional)
        match dotenvy::dotenv() {
            Ok(_) => (),
            Err(e) => {
                // .env file is optional, so we only warn if it's not found
                eprintln!("Warning: .env file not found or couldn't be loaded: {}", e);
            }
        }
        Ok(())
    }

    /// Get a required environment variable or panic with a helpful message.
    fn get_required_env(key: &str) -> String {
        env::var(key).unwrap_or_else(|_| {
            panic!(
                "Environment variable {} is required but not set. Please check your .env file.",
                key
            )
        })
    }

    /// Get an optional environment variable.
    fn get_optional_env(key: &str) -> Option<String> {
        env::var(key).ok()
    }

    /// Load API configuration from both environment variables (.env) and config.yaml.
    /// 
    /// Sensitive data (credentials, API keys, URLs) are loaded from environment variables,
    /// while application behavior settings are loaded from config.yaml.
    pub fn from_env_and_config() -> Result<Self, Box<dyn Error>> {
        // Load environment variables from .env file
        Self::load_env()?;

        // Load non-sensitive settings from config.yaml
        let mut config = if std::path::Path::new("config.yaml").exists() {
            let config_contents = fs::read_to_string("config.yaml")?;
            let yaml_config: HashMap<String, serde_yaml::Value> =
                serde_yaml::from_str(&config_contents)?;
            
            if let Some(api_config_value) = yaml_config.get("ig_trading_api") {
                serde_yaml::from_value::<ApiConfig>(api_config_value.clone())?
            } else {
                // If config.yaml doesn't have ig_trading_api section, use defaults
                ApiConfig::new()
            }
        } else {
            // If config.yaml doesn't exist, use defaults for non-sensitive settings
            eprintln!("Warning: config.yaml not found. Using default values for application settings.");
            ApiConfig::new()
        };

        // Override with environment variables for sensitive data
        config.api_key = Self::get_required_env("IG_API_KEY");
        config.username = Self::get_required_env("IG_USERNAME");
        config.password = Self::get_required_env("IG_PASSWORD");
        config.account_number_demo = Self::get_required_env("IG_ACCOUNT_NUMBER_DEMO");
        config.account_number_live = Self::get_required_env("IG_ACCOUNT_NUMBER_LIVE");
        config.account_number_test = Self::get_optional_env("IG_ACCOUNT_NUMBER_TEST");
        config.base_url_demo = Self::get_required_env("IG_BASE_URL_DEMO");
        config.base_url_live = Self::get_required_env("IG_BASE_URL_LIVE");
        
        // Parse execution environment
        let env_str = Self::get_required_env("IG_EXECUTION_ENVIRONMENT");
        config.execution_environment = ExecutionEnvironment::from_str(&env_str)?;

        Ok(config)
    }
}

/// Load the API configuration from both environment variables and config file.
/// 
/// This is the recommended way to create an ApiConfig instance.
/// It loads sensitive data from environment variables (.env file) and
/// non-sensitive application settings from config.yaml.
impl Default for ApiConfig {
    fn default() -> Self {
        Self::from_env_and_config().unwrap_or_else(|e| {
            panic!("Failed to load API configuration: {}", e);
        })
    }
}

/// Struct to represent API errors.
#[derive(Debug)]
pub struct ApiError {
    /// The error message.
    pub message: String,
}

/// Implement the Display trait for ApiError to provide custom string representation.
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write the error message to the formatter.
        write!(f, "API error: {}", self.message)
    }
}

/// Implement the Error trait for ApiError to handle errors.
impl std::error::Error for ApiError {}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// UTILITY FUNCTIONS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Convert a serializable object representing GET parameters to a query string.
pub fn params_to_query_string<T: Serialize>(
    data: &T,
) -> Result<String, serde_urlencoded::ser::Error> {
    serde_urlencoded::to_string(data)
}

/// Convert a HeaderMap to a JSON serde_json::Value.
pub fn headers_to_json(headers: &HeaderMap) -> Result<Value, Box<dyn Error>> {
    let mut map = serde_json::Map::new();

    for (key, value) in headers {
        map.insert(
            key.as_str().to_string(),
            Value::String(value.to_str()?.to_string()),
        );
    }

    Ok(Value::Object(map))
}
