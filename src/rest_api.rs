use crate::common::*;
use crate::rest_client::*;
use crate::rest_models::*;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

/// Struct to encapsulate the API and its configuration.
#[derive(Clone, Debug)]
pub struct RestApi {
    /// The HTTP client for making requests.
    pub client: RestClient,
    /// The API configuration.
    pub config: ApiConfig,
}

/// Provide an implementation for the Api struct.
impl RestApi {
    pub async fn new(config: ApiConfig) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: RestClient::new(config.clone()).await?,
            config,
        })
    }

    /// Get session details for the current session.
    pub async fn get_session(
        &self,
        params: Option<HashMap<String, String>>,
    ) -> Result<(Value, Session), Box::<dyn Error>> {
        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .get("session".to_string(), Some(1), params)
            .await?;

        // Convert header_map to json
        let headers: Value = headers_to_json(&header_map)?;
        // Convert the serde_json::Value response to Session model.
        let session: Session = serde_json::from_value(response_value)?;

        Ok((headers, session))
    }

    /// Log out of the IG API by deleting the current session.
    pub async fn delete_session(&self) -> Result<(Value, ()), Box<dyn Error>> {
        // Send the request to the REST client.
        let (header_map, _) = self
            .client
            .delete("session".to_string())
            .await?;

        // Convert header_map to json
        let headers: Value = headers_to_json(&header_map)?;

        Ok((headers, ()))
    }
}
