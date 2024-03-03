use crate::common::*;
use crate::rest_client::*;
use crate::rest_models::*;
use serde_json::Value;
use std::error::Error;

/// Struct to encapsulate the API, including the REST HTTP client, the API configuration
/// and all the methods to interact with the IG REST API.
#[derive(Clone, Debug)]
pub struct RestApi {
    /// The HTTP client for making requests.
    pub client: RestClient,
    /// The API configuration.
    pub config: ApiConfig,
}

/// Provide an implementation for the Api struct with all the methods to interact with the IG REST API.
impl RestApi {
    /// Create a new instance of the RestApi struct based on the provided configuration.
    pub async fn new(config: ApiConfig) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: RestClient::new(config.clone()).await?,
            config,
        })
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////
    //
    // ACCOUNT METHODS.
    //
    ////////////////////////////////////////////////////////////////////////////////////////////////////////

    /// Returns a list of the logged-in client's accounts.
    pub async fn accounts_get(
        &self,
    ) -> Result<(Value, AccountsResponse), Box<dyn Error>> {
        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .get("accounts".to_string(), Some(1), &None::<Empty>)
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Convert the serde_json::Value response to Session model.
        let accounts = AccountsResponse::from_value(&response_value)?;

        Ok((headers, accounts))
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////
    //
    // SESSION METHODS.
    //
    ////////////////////////////////////////////////////////////////////////////////////////////////////////

    /// Log out of the IG API by deleting the current session.
    pub async fn session_delete(&self) -> Result<(Value, ()), Box<dyn Error>> {
        // Send the request to the REST client.
        let (header_map, _) = self.client.delete("session".to_string()).await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;

        Ok((headers, ()))
    }
    
    /// Get session details for the current session.
    pub async fn session_get(
        &self,
        params: Option<SessionDetailsRequest>,
    ) -> Result<(Value, SessionDetailsResponse), Box<dyn Error>> {
        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .get("session".to_string(), Some(1), &params)
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Convert the serde_json::Value response to Session model.
        let session = SessionDetailsResponse::from_value(&response_value)?;

        Ok((headers, session))
    }

    /// This method is not implemented as the login process is handled by the rest_client module.
    pub async fn session_post() {
        unimplemented!("This method will not be implemented as the login process is handled by the rest_client module.");
    }

    /// Switch to a different account by updating the current session.
    pub async fn session_put(
        &self,
        body: &AccountSwitchRequest,
    ) -> Result<(Value, AccountSwitchResponse), Box<dyn Error>> {
        // Validate the body.
        body.validate()?;

        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .put("session".to_string(), body, Some(1))
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Deserialize the response_value to AccountSwitchResponse.
        let account_switch_response = AccountSwitchResponse::from_value(&response_value)?;

        Ok((headers, account_switch_response))
    }

    /// Creates a trading session, obtaining session tokens for subsequent API access.
    /// Please note, region-specific login restrictions may apply.
    pub async fn session_encryption_key_get(
        &self,
    ) -> Result<(Value, SessionEncryptionKeyResponse), Box<dyn Error>> {
        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .get("session/encryptionKey".to_string(), Some(1), &None::<Empty>)
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Deserialize the response_value to EncryptionKeyResponse.
        let encryption_key_response = SessionEncryptionKeyResponse::from_value(&response_value)?;

        Ok((headers, encryption_key_response))
    }

    pub async fn session_refresh_token_post(
        &self,
        body: &SessionRefreshTokenRequest,
    ) -> Result<(Value, SessionRefreshTokenResponse), Box<dyn Error>> {
        // Validate the body.
        body.validate()?;

        // Send the request to the REST client.
        let (headers, response_value) = self
            .client
            .post("session/refresh-token".to_string(), Some(1), body)
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&headers)?;
        // Deserialize the response_value to SessionRefreshTokenResponse.
        let session_refresh_token_response = SessionRefreshTokenResponse::from_value(&response_value)?;

        Ok((headers, session_refresh_token_response))
    }

}
