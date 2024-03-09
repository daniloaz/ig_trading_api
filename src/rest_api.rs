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
    ) -> Result<(Value, AccountsGetResponse), Box<dyn Error>> {
        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .get("accounts".to_string(), Some(1), &None::<Empty>)
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Convert the serde_json::Value response to Session model.
        let accounts = AccountsGetResponse::from_value(&response_value)?;

        Ok((headers, accounts))
    }

    /// Returns account preferences.
    pub async fn accounts_preferences_get(
        &self,
    ) -> Result<(Value, AccountsPreferencesGetResponse), Box<dyn Error>> {
        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .get("accounts/preferences".to_string(), Some(1), &None::<Empty>)
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Convert the serde_json::Value response to Session model.
        let accounts_preferences = AccountsPreferencesGetResponse::from_value(&response_value)?;

        Ok((headers, accounts_preferences))
    }

    /// Updates account preferences.
    pub async fn accounts_preferences_put(
        &self,
        body: &AccountsPreferencesPutRequest,
    ) -> Result<(Value, AccountsPreferencesStatusPutResponse), Box<dyn Error>> {
        // Validate the body.
        body.validate()?;

        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .put("accounts/preferences".to_string(), Some(1), body)
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Convert the serde_json::Value response to Session model.
        let status = AccountsPreferencesStatusPutResponse::from_value(&response_value)?;

        Ok((headers, status))
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////
    //
    // CONFIRMS METHODS.
    //
    ////////////////////////////////////////////////////////////////////////////////////////////////////////

    /// Returns a deal confirmation for the given deal reference. Please note, this
    /// should only be used if the deal confirmation isn't received via the streaming API.
    pub async fn confirms_get(
        &self,
        params: ConfirmsGetRequest,
    ) -> Result<(Value, ConfirmsGetResponse), Box<dyn Error>> {

        let url = format!("confirms/{}", params.deal_reference);

        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .get(url, Some(1), &None::<Empty>)
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Convert the serde_json::Value response to Session model.
        let confirmations = ConfirmsGetResponse::from_value(&response_value)?;

        Ok((headers, confirmations))
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////
    //
    // HISTORY METHODS.
    //
    ////////////////////////////////////////////////////////////////////////////////////////////////////////

    /// Returns the account activity history.
    pub async fn history_activity_get(
        &self,
        params: ActivityHistoryGetRequest,
    ) -> Result<(Value, ActivityHistoryGetResponse), Box<dyn Error>> {
        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .get("history/activity".to_string(), Some(3), &Some(params))
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Convert the serde_json::Value response to Session model.
        let history_activity = ActivityHistoryGetResponse::from_value(&response_value)?;

        Ok((headers, history_activity))
    }

    /// Returns the transaction history. Returns the minute prices within the last 10 minutes by default.
    pub async fn history_transactions_get(
        &self,
        params: TransactionHistoryGetRequest,
    ) -> Result<(Value, TransactionHistoryGetResponse), Box<dyn Error>> {
        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .get("history/transactions".to_string(), Some(2), &Some(params))
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Convert the serde_json::Value response to Session model.
        let history_activity = TransactionHistoryGetResponse::from_value(&response_value)?;

        Ok((headers, history_activity))
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////
    //
    // POSITIONS METHODS.
    //
    ////////////////////////////////////////////////////////////////////////////////////////////////////////

    /// Returns a specific open position for the active account.
    pub async fn position_delete(
        &self,
        body: PositionDeleteRequest,
    ) -> Result<(Value, PositionDeleteResponse), Box<dyn Error>> {

        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .delete("positions/otc".to_string(), Some(1), &Some(body))
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Convert the serde_json::Value response to Session model.
        let position_delete_response = PositionDeleteResponse::from_value(&response_value)?;

        Ok((headers, position_delete_response))
    }

    /// Returns a specific open position for the active account.
    pub async fn position_get(
        &self,
        params: PositionGetRequest,
    ) -> Result<(Value, PositionGetResponse), Box<dyn Error>> {

        // Create the url based on the params.
        let url = format!("positions/{}", params.deal_id);

        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .get(url, Some(2), &None::<Empty>)
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Convert the serde_json::Value response to Session model.
        let positions = PositionGetResponse::from_value(&response_value)?;

        Ok((headers, positions))
    }

    /// Returns all open positions for the active account.
    pub async fn positions_get(
        &self,
    ) -> Result<(Value, PositionsGetResponse), Box<dyn Error>> {

        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .get("positions".to_string(), Some(2), &None::<Empty>)
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Convert the serde_json::Value response to Session model.
        let positions = PositionsGetResponse::from_value(&response_value)?;

        Ok((headers, positions))
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////
    //
    // SESSION METHODS.
    //
    ////////////////////////////////////////////////////////////////////////////////////////////////////////

    /// Log out of the IG API by deleting the current session.
    pub async fn session_delete(&self) -> Result<(Value, ()), Box<dyn Error>> {
        // Send the request to the REST client.
        let (header_map, _) = self.client.delete("session".to_string(), Some(1), &None::<Empty>).await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;

        Ok((headers, ()))
    }
    
    /// Get session details for the current session.
    pub async fn session_get(
        &self,
        params: Option<SessionDetailsGetRequest>,
    ) -> Result<(Value, SessionDetailsGetResponse), Box<dyn Error>> {
        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .get("session".to_string(), Some(1), &params)
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Convert the serde_json::Value response to Session model.
        let session = SessionDetailsGetResponse::from_value(&response_value)?;

        Ok((headers, session))
    }

    /// This method is not implemented as the login process is handled by the rest_client module.
    pub async fn session_post() {
        unimplemented!("This method will not be implemented as the login process is handled by the rest_client module.");
    }

    /// Switch to a different account by updating the current session.
    pub async fn session_put(
        &self,
        body: &AccountSwitchPutRequest,
    ) -> Result<(Value, AccountSwitchPutResponse), Box<dyn Error>> {
        // Validate the body.
        body.validate()?;

        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .put("session".to_string(), Some(1), body)
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Deserialize the response_value to AccountSwitchResponse.
        let account_switch_response = AccountSwitchPutResponse::from_value(&response_value)?;

        Ok((headers, account_switch_response))
    }

    /// Creates a trading session, obtaining session tokens for subsequent API access.
    /// Please note, region-specific login restrictions may apply.
    pub async fn session_encryption_key_get(
        &self,
    ) -> Result<(Value, SessionEncryptionKeyGetResponse), Box<dyn Error>> {
        // Send the request to the REST client.
        let (header_map, response_value) = self
            .client
            .get("session/encryptionKey".to_string(), Some(1), &None::<Empty>)
            .await?;

        // Convert header_map to json.
        let headers: Value = headers_to_json(&header_map)?;
        // Deserialize the response_value to EncryptionKeyResponse.
        let encryption_key_response = SessionEncryptionKeyGetResponse::from_value(&response_value)?;

        Ok((headers, encryption_key_response))
    }

    pub async fn session_refresh_token_post(
        &self,
        body: &SessionRefreshTokenPostRequest,
    ) -> Result<(Value, SessionRefreshTokenPostResponse), Box<dyn Error>> {
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
        let session_refresh_token_response = SessionRefreshTokenPostResponse::from_value(&response_value)?;

        Ok((headers, session_refresh_token_response))
    }

}
