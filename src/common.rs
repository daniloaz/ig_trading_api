use reqwest::header::HeaderMap;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ApiConfig {
    /// Your demo IG account number.
    pub account_number_demo: String,
    /// Your live IG account number.
    pub account_number_live: String,
    /// An alternative account number for testing. It must be associated to
    /// the same running environment as the main account number.
    pub account_number_test: Option<String>,
    /// The API key assigned to your IG account.
    pub api_key: String,
    /// Automatically log in to the API on instantiation and when the session expires.
    pub auto_login: Option<bool>,
    /// The base URL for the demo environment.
    pub base_url_demo: String,
    /// The base URL for the live environment.
    pub base_url_live: String,
    /// The execution environment (DEMO or LIVE).
    pub execution_environment: ExecutionEnvironment,
    /// Your user password.
    pub password: String,
    /// The session version to use for login requests.
    pub session_version: Option<usize>,
    /// Your username.
    pub username: String,
}

/// Load the API configuration from the config value within the configuration file.
impl Default for ApiConfig {
    fn default() -> Self {
        // Panic if the config file is not found.
        if !std::path::Path::new("config.yaml").exists() {
            panic!("config.yaml file not found!");
        }
        let config_contents = fs::read_to_string("config.yaml").unwrap();
        let config: HashMap<String, serde_yaml::Value> =
            serde_yaml::from_str(&config_contents).unwrap();
        let api_config_value = config
            .get("ig_trading_api")
            .expect("ig_trading_api value not found in config file!");
        let api_config: ApiConfig = serde_yaml::from_value(api_config_value.clone()).unwrap();

        api_config
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
pub fn params_to_query_string<T: Serialize>(data: &T) -> Result<String, serde_urlencoded::ser::Error> {
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