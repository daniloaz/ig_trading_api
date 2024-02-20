use crate::common::*;
use reqwest::StatusCode;
use serde_json::{json, Value};

/// Struct to encapsulate the API and its configuration.
#[derive(Clone, Debug)]
pub struct RestApi {
    /// The HTTP client for making requests.
    pub client: reqwest::Client,
    /// The API configuration.
    pub config: ApiConfig,
    /// Client session security access token.
    pub cst: Option<String>,
    /// X-SECURITY-TOKEN header value (Account session security access token).
    pub x_security_token: Option<String>,
}

/// Provide an implementation for the Api struct.
impl RestApi {
    pub fn new(config: ApiConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
            cst: None,
            x_security_token: None,
        }
    }

    /// Log in to the IG API.
    pub async fn login_v2(&mut self) -> Result<Value, ApiError> {
        // Create the login request body.
        let login_request = json!({
            "identifier": self.config.username.clone(),
            "password": self.config.password.clone(),
        });

        // Determine the API URL based on the account type.
        let api_url = match self.config.account_type {
            AccountType::Demo => &self.config.base_url_demo,
            AccountType::Live => &self.config.base_url_live,
        };

        // Send the login request.
        let response = self
            .client
            .post(&format!("{}/session", api_url))
            .json(&login_request)
            .header("Accept", "application/json; charset=UTF-8")
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("X-IG-API-KEY", &self.config.api_key)
            .header("Version", "2")
            .send()
            .await?;

        // Check the response status code.
        match response.status() {
            // If the status code is 200 OK, return the JSON body plus headers.
            StatusCode::OK => {
                let headers = headers_to_json(&response.headers());
                let mut json: Value = response.json().await?;
                json["headers"] = headers;

                // Get CST and X-SECURITY-TOKEN from the login response.
                let cst: Option<String> = json
                    .get("headers")
                    .and_then(|h| h.get("cst"))
                    .and_then(|cst| cst.as_str())
                    .map(|s| s.to_string());
                let x_security_token: Option<String> = json
                    .get("headers")
                    .and_then(|h| h.get("x-security-token"))
                    .and_then(|token| token.as_str())
                    .map(|s| s.to_string());

                // Set the CST and X-SECURITY-TOKEN values in the struct.
                self.cst = cst;
                self.x_security_token = x_security_token;

                // If the CST and X-SECURITY-TOKEN values are not present, return an error.
                if self.cst.is_none() || self.x_security_token.is_none() {
                    return Err(ApiError {
                        message: "CST and X-SECURITY-TOKEN not found in login response headers."
                            .to_string(),
                    });
                }

                Ok(json)
            }
            // If the status code is not 200 OK, return an error.
            _ => Err(ApiError {
                message: format!("Login failed with status code: {}", response.status()),
            }),
        }
    }

    /// Log out of the IG API.
    pub async fn logout(&self) -> Result<(), ApiError> {
        // Determine the API URL based on the account type.
        let api_url = match self.config.account_type {
            AccountType::Demo => &self.config.base_url_demo,
            AccountType::Live => &self.config.base_url_live,
        };

        // Send the logout request.
        let response = self
            .client
            .delete(&format!("{}/session", api_url))
            .header("Accept", "application/json; charset=UTF-8")
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("CST", self.cst.as_ref().unwrap_or(&"".to_string()))
            .header("Version", "1")
            .header("X-IG-API-KEY", &self.config.api_key)
            .header(
                "X-SECURITY-TOKEN",
                self.x_security_token.as_ref().unwrap_or(&"".to_string()),
            )
            .send()
            .await?;

        // Check the response status code.
        match response.status() {
            // If the status code is 204 No Content, return success.
            StatusCode::NO_CONTENT => Ok(()),
            // If the status code is not 204 No Content, return an error.
            _ => Err(ApiError {
                message: format!("Logout failed with status code: {}", response.status()),
            }),
        }
    }
}
