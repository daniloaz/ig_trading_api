use chrono::{NaiveDate, ParseError};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{from_value, to_string, Value};
use serde_path_to_error::deserialize;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::string::ToString;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum AccountType {
    Demo,
    Live,
}

impl<'de> Deserialize<'de> for AccountType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_uppercase();

        match s.as_str() {
            "DEMO" => Ok(AccountType::Demo),
            "LIVE" => Ok(AccountType::Live),
            _ => Err(serde::de::Error::custom("Invalid account type")),
        }
    }
}

impl FromStr for AccountType {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DEMO" => Ok(AccountType::Demo),
            "LIVE" => Ok(AccountType::Live),
            _ => Err(ApiError {
                message: format!("Invalid account type: {}", s),
            }),
        }
    }
}

/// Struct to hold IG API configuration data.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ApiConfig {
    /// Your IG account number.
    pub account_number: String,
    /// The account type (DEMO or LIVE).
    pub account_type: AccountType,
    /// The API key assigned to your IG account.
    pub api_key: String,
    /// The base URL for the demo environment.
    pub base_url_demo: String,
    /// The base URL for the live environment.
    pub base_url_live: String,
    /// Your user password.
    pub password: String,
    /// Your username.
    pub username: String,
}

/// Load the API configuration from the config value within the configuration file.
impl Default for ApiConfig {
    fn default() -> Self {
        let config_contents = fs::read_to_string("config.yaml").unwrap();
        let config: HashMap<String, serde_yaml::Value> =
            serde_yaml::from_str(&config_contents).unwrap();
        let api_config_value = config
            .get("IG")
            .expect("IG value not found in config file!");
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

/// Implement the `From<ParseError>` trait for ApiError.
impl From<ParseError> for ApiError {
    /// Convert ParseError to ApiError
    fn from(err: ParseError) -> Self {
        ApiError {
            message: format!("Parse error: {}", err),
        }
    }
}

/// Implement the `From<reqwest::Error>` trait for ApiError.
impl From<reqwest::Error> for ApiError {
    /// Convert reqwest::Error to ApiError
    fn from(error: reqwest::Error) -> Self {
        ApiError {
            message: format!("Request failed: {}", error),
        }
    }
}

/// Implement the `From<serde_json::Error>` trait for ApiError.
impl From<serde_json::Error> for ApiError {
    /// Convert serde_json::Error to ApiError
    fn from(error: serde_json::Error) -> Self {
        ApiError {
            message: format!("JSON Error: {}", error.to_string()),
        }
    }
}

/// Implement the `From<String>` trait for ApiError.
impl From<String> for ApiError {
    /// Convert String to ApiError
    fn from(s: String) -> Self {
        ApiError { message: s }
    }
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

/// Convert a HeaderMap to a JSON Value.
pub fn headers_to_json(headers: &HeaderMap) -> Value {
    let mut map = serde_json::Map::new();

    for (key, value) in headers {
        map.insert(
            key.as_str().to_string(),
            Value::String(value.to_str().unwrap_or("").to_string()),
        );
    }

    Value::Object(map)
}
