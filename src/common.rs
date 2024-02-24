use reqwest::header::HeaderMap;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
//use serde_path_to_error::deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;
use std::string::ToString;

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

/// Struct to hold IG API configuration data.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ApiConfig {
    /// Your demo IG account number.
    pub account_number_demo: String,
    /// Your live IG account number.
    pub account_number_live: String,
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

///
/// UTILITY FUNCTIONS
///

/// Improved deserialization function that provides better error messages using serde_path_to_error.
pub fn deserialize<'de, T>(value: &'de Value) -> Result<T, Box<dyn Error>>
where
    T: Deserialize<'de>,
{
    // Deserialize the value using serde_path_to_error.
    let result = serde_path_to_error::deserialize(value);

    // Return the result.
    match result {
        Ok(value) => Ok(value),
        Err(e) => Err(Box::new(ApiError {
            message: format!("Failed to deserialize JSON serde_json::Value: {}", e),
        })),
    }
}

/// Convert a HashMap of parameters to a query string for use in a URL.
pub fn params_to_query_string(params: Option<HashMap<String, String>>) -> String {
    let mut query_string = "".to_string();

    if let Some(params) = params {
        if !params.is_empty() {
            if query_string.is_empty() {
                query_string.push('?');
            }

            for (key, value) in params {
                query_string.push_str(&format!("{}={}&", key, value));
            }

            // Remove the trailing ampersand (&).
            query_string.pop();
        }
    }

    query_string
}

/// Convert a HashMap of parameters to a JSON serde_json::Value.
pub fn params_to_json(params: Option<HashMap<String, String>>) -> Value {
    let mut map = serde_json::Map::new();

    if let Some(params) = params {
        if !params.is_empty() {
            for (key, value) in params {
                map.insert(key, Value::String(value));
            }
        }
    }

    Value::Object(map)
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