use crate::common::*;
use crate::rest_models::{
    AuthenticationRequest, AuthenticationResponseV3, ValidateRequest, ValidateResponse,
};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::Value;
use std::error::Error;

/// Default session version if not explicitly set.
const DEFAULT_SESSION_VERSION: usize = 2;
/// Default auto-login behavior if not explicitly set.
const DEFAULT_AUTO_LOGIN: bool = true;

/// Struct to represent the REST API client.
#[derive(Clone, Debug)]
pub struct RestClient {
    /// The API authentication headers.
    pub auth_headers: Option<HeaderMap>,
    /// Automatically log in to the API on instantiation and when the session expires.
    pub auto_login: bool,
    /// The API base URL based on the account type.
    pub base_url: String,
    /// The reqwest client instance.
    pub client: reqwest::Client,
    /// Common headers used for all requests.
    pub common_headers: HeaderMap,
    /// The API configuration.
    pub config: ApiConfig,
    /// The refresh token to use for refreshing the session when session_version is 3.
    pub refresh_token: Option<String>,
    /// Session version.
    pub session_version: usize,
}

/// Implementation for the RestClient struct.
impl RestClient {
    /// Send a DELETE request to the API.
    pub async fn delete(&self, method: String) -> Result<(HeaderMap, ()), Box<dyn Error>> {
        // Default API version is 1.
        let api_version: usize = 1;

        let response = self
            .client
            .delete(&format!("{}/{}", &self.base_url, method))
            .headers(self.auth_headers.clone().unwrap_or(HeaderMap::new()))
            .headers(self.common_headers.clone())
            .header("Version", api_version)
            .send()
            .await?;

        // Check the response status code.
        match response.status() {
            // If the status code is 204 No Content, return success.
            StatusCode::NO_CONTENT => Ok((response.headers().clone(), ())),
            // If the status code is not 204 No Content, return an error.
            _ => Err(Box::new(ApiError {
                message: format!(
                    "DELETE operation using method '{}' failed with status code: {:?}",
                    method,
                    response.status()
                ),
            })),
        }
    }

    /// Create a new RestClient instance.
    pub async fn new(config: ApiConfig) -> Result<Self, Box<dyn Error>> {
        // Determine the API base URL based on the account type.
        let base_url = match config.execution_environment {
            ExecutionEnvironment::Demo => config.base_url_demo.clone(),
            ExecutionEnvironment::Live => config.base_url_live.clone(),
        };

        // Default session version is DEFAULT_SESSION_VERSION.
        let session_version = config.session_version.unwrap_or(DEFAULT_SESSION_VERSION);
        // Default auto_login is DEFAULT_AUTO_LOGIN.
        let auto_login = config.auto_login.unwrap_or(DEFAULT_AUTO_LOGIN);

        // Set the common headers.
        let mut common_headers = HeaderMap::new();
        common_headers.insert("Accept", "application/json; charset=UTF-8".parse()?);
        common_headers.insert("Content-Type", "application/json; charset=UTF-8".parse()?);
        common_headers.insert("X-IG-API-KEY", config.api_key.as_str().parse()?);

        // Create a new RestClient instance.
        let mut rest_client = Self {
            auth_headers: None,
            auto_login,
            base_url,
            client: reqwest::Client::new(),
            common_headers,
            config,
            refresh_token: None,
            session_version,
        };

        // If auto_login is true, then login to the API.
        if auto_login {
            rest_client.login().await?;
        };

        Ok(rest_client)
    }

    /// Send a GET request to the API.
    pub async fn get(
        &self,
        method: String,
        api_version: Option<usize>,
        params: &Option<impl Serialize + ValidateRequest>,
    ) -> Result<(HeaderMap, Value), Box<dyn Error>> {
        // Default API version is 1.
        let api_version = api_version.unwrap_or(1).to_string();
        // Validate the params.
        if let Some(params) = params {
            params.validate()?;
        }
        // Convert params to a query string.
        let query_string = params_to_query_string(params)?;

        let response = self
            .client
            .get(&format!("{}/{}?{}", &self.base_url, method, query_string))
            .headers(self.auth_headers.clone().unwrap_or(HeaderMap::new()))
            .headers(self.common_headers.clone())
            .header("Version", api_version)
            .send()
            .await?;

        // Check the response status code.
        match response.status() {
            // If the status code is 200 OK, return the JSON body.
            StatusCode::OK => Ok((response.headers().clone(), response.json().await?)),
            // If the status code is not 200 OK, return an error.
            _ => Err(Box::new(ApiError {
                message: format!(
                    "GET operation using method '{}' and query_string '{}' failed with status code: {:?}",
                    method,
                    query_string,
                    response.status()
                ),
            })),
        }
    }

    /// Log in to the REST API.
    pub async fn login(&mut self) -> Result<Value, Box<dyn Error>> {
        println!("Logging in with session version: {}", self.session_version);

        match self.session_version {
            1 | 2 => Ok(self.login_v2().await?),
            3 => Ok(self.login_v3().await?),
            _ => Err(Box::new(ApiError {
                message: format!("Invalid session version: {}", self.session_version),
            })),
        }
    }

    /// Log in to the REST API using session version 2.
    pub async fn login_v2(&mut self) -> Result<Value, Box<dyn Error>> {
        // Create the login request body.
        let login_request_body = AuthenticationRequest {
            identifier: self.config.username.clone(),
            password: self.config.password.clone(),
        };

        // Validate the login request body.
        login_request_body.validate()?;

        // Send the login request.
        let response = self
            .client
            .post(&format!("{}/session", &self.base_url))
            .json(&login_request_body)
            .headers(self.common_headers.clone())
            .header("Version", "2")
            .send()
            .await?;

        // Check the response status code.
        match response.status() {
            // If the status code is 200 OK, return the JSON body plus headers.
            StatusCode::OK => {
                // Get cst and x-security-token headers from the login response.
                let mut auth_headers = HeaderMap::new();
                if let Some(cst_header) = response.headers().get("cst") {
                    auth_headers.insert("cst", HeaderValue::from_str(cst_header.to_str()?)?);
                }
                if let Some(security_token_header) = response.headers().get("x-security-token") {
                    auth_headers.insert(
                        "x-security-token",
                        HeaderValue::from_str(security_token_header.to_str()?).unwrap(),
                    );
                }

                // If any of the auth_headers doesn't exist, return an error.
                if auth_headers.get("cst").is_none()
                    || auth_headers.get("x-security-token").is_none()
                {
                    return Err(Box::new(ApiError {
                        message:
                            "Any of the cst / x-security-token headers not found in login response."
                                .to_string(),
                    }));
                }

                self.auth_headers = Some(auth_headers);

                Ok(response.json().await?)
            }
            // If the status code is not 200 OK, return an error.
            _ => Err(Box::new(ApiError {
                message: format!("Login failed with status code: {:?}", response.status()),
            })),
        }
    }

    /// Log in to the REST API using session version 2.
    pub async fn login_v3(&mut self) -> Result<Value, Box<dyn Error>> {
        // Create the login request body.
        let login_request_body = AuthenticationRequest {
            identifier: self.config.username.clone(),
            password: self.config.password.clone(),
        };

        // Validate the login request body.
        login_request_body.validate()?;

        // Send the login request.
        let response = self
            .client
            .post(&format!("{}/session", &self.base_url))
            .json(&login_request_body)
            .headers(self.common_headers.clone())
            .header("Version", "3")
            .send()
            .await?;

        // Check the response status code.
        match response.status() {
            // If the status code is 200 OK, return the JSON body plus headers.
            StatusCode::OK => {
                // Deserialize the response body to a LoginResponseV3.
                let response_body = response.json().await?;
                let login_response = AuthenticationResponseV3::from_value(&response_body)?;

                // Get access_token from the login response and set it as the Bearer token in Authorization header.
                let mut auth_headers = HeaderMap::new();
                auth_headers.insert(
                    "Authorization",
                    HeaderValue::from_str(&format!(
                        "Bearer {}",
                        login_response.oauth_token.access_token
                    ))?,
                );

                let account_number = match self.config.execution_environment {
                    ExecutionEnvironment::Demo => self.config.account_number_demo.clone(),
                    ExecutionEnvironment::Live => self.config.account_number_live.clone(),
                };

                auth_headers.insert("IG-ACCOUNT-ID", HeaderValue::from_str(&account_number)?);

                self.auth_headers = Some(auth_headers);

                self.refresh_token = Some(login_response.oauth_token.refresh_token);

                Ok(response_body)
            }
            // If the status code is not 200 OK, return an error.
            _ => Err(Box::new(ApiError {
                message: format!("Login failed with status code: {:?}", response.status()),
            })),
        }
    }

    /// Send a POST request to the REST API.
    pub async fn post(
        &self,
        method: String,
        version: Option<usize>,
        body: &(impl Serialize + ValidateRequest),
    ) -> Result<(HeaderMap, Value), Box<dyn Error>> {
        // Default API version is 1.
        let version = version.unwrap_or(1).to_string();
        // Validate the body.
        body.validate()?;
        // Convert the body to a serde_json::Value.
        let body = serde_json::to_value(body)?;

        let response = self
            .client
            .post(&format!("{}/{}", &self.base_url, method))
            .json(&body)
            .headers(self.auth_headers.clone().unwrap_or(HeaderMap::new()))
            .headers(self.common_headers.clone())
            .header("Version", version.clone())
            .send()
            .await?;

        // Check the response status code.
        match response.status() {
            // If the status code is 200 OK, return the JSON body.
            StatusCode::OK => Ok((response.headers().clone(), response.json().await?)),
            // If the status code is not 200 OK, return an error.
            _ => Err(Box::new(ApiError {
                message: format!(
                    "POST operation using method '{}', version '{}' and body '{:?}' failed with status code: {:?}",
                    method,
                    version,
                    body,
                    response.status()
                ),
            })),
        }
    }

    /// Send a PUT request to the REST API.
    pub async fn put(
        &self,
        method: String,
        body: &(impl Serialize + ValidateRequest),
        version: Option<usize>,
    ) -> Result<(HeaderMap, Value), Box<dyn Error>> {
        // Default API version is 1.
        let version = version.unwrap_or(1).to_string();
        // Validate the body.
        body.validate()?;

        // Send the PUT request.
        let response = self
            .client
            .put(&format!("{}/{}", &self.base_url, method))
            .json(&body)
            .headers(self.auth_headers.clone().unwrap_or(HeaderMap::new()))
            .headers(self.common_headers.clone())
            .header("Version", version.clone())
            .send()
            .await?;

        // Check the response status code.
        match response.status() {
            // If the status code is 200 OK, return the JSON body.
            StatusCode::OK => Ok((response.headers().clone(), response.json().await?)),
            // If the status code is not 200 OK, return an error.
            _ => Err(Box::new(ApiError {
                message: format!(
                    "PUT operation using method '{}', version '{}' and body '{:?}' failed with status code: {}",
                    method,
                    version,
                    serde_json::to_string(&body)?,
                    response.status()
                ),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ApiConfig, ExecutionEnvironment};

    #[tokio::test]
    async fn new_rest_client_works() {
        // Create a mock API configuration
        let config = ApiConfig {
            account_number_demo: "test_account_number_demo".to_string(),
            account_number_live: "test_account_number_live".to_string(),
            account_number_test: None,
            api_key: "test_api_key".to_string(),
            auto_login: Some(false),
            execution_environment: ExecutionEnvironment::Demo,
            base_url_demo: "https://demo.example.com".to_string(),
            base_url_live: "https://live.example.com".to_string(),
            session_version: Some(2),
            password: "test_password".to_string(),
            username: "test_username".to_string(),
        };

        // Call the `new` function with the mock configuration
        let rest_client = RestClient::new(config).await.unwrap();

        // Make assertions about the returned `RestClient` object
        assert_eq!(rest_client.auth_headers, None);
        assert_eq!(rest_client.auto_login, false);
        assert_eq!(rest_client.base_url, "https://demo.example.com");
        assert_eq!(
            rest_client.common_headers.get("X-IG-API-KEY").unwrap(),
            "test_api_key"
        );
        assert_eq!(
            rest_client.config.account_number_demo,
            "test_account_number_demo"
        );
        assert_eq!(
            rest_client.config.account_number_live,
            "test_account_number_live"
        );
        assert_eq!(rest_client.config.account_number_test, None);
        assert_eq!(rest_client.config.api_key, "test_api_key");
        assert_eq!(rest_client.config.auto_login, Some(false));
        assert_eq!(
            rest_client.config.execution_environment,
            ExecutionEnvironment::Demo
        );
        assert_eq!(rest_client.config.base_url_demo, "https://demo.example.com");
        assert_eq!(rest_client.config.base_url_live, "https://live.example.com");
        assert_eq!(rest_client.config.session_version, Some(2));
        assert_eq!(rest_client.config.password, "test_password");
        assert_eq!(rest_client.config.username, "test_username");
        assert_eq!(rest_client.session_version, 2);
    }
}
