use base64::{display::Base64Display, engine::general_purpose::STANDARD};
use ig_trading_api::common::*;
use ig_trading_api::rest_api::*;
use ig_trading_api::rest_models::*;
use regex::Regex;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::OnceCell;

static API: OnceCell<Arc<RestApi>> = OnceCell::const_new();
static DEFAULT_TEST_DELAY_SECONDS: u64 = 3;

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// FUNCTIONS SHARED BY ALL THE INTEGRATION TESTS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// If the API instance is not already initialized, then create and initialize a new one.
/// Otherwise, return the existing instance.
async fn get_or_init_rest_api() -> Arc<RestApi> {
    API.get_or_init(|| async {
        // Load the configuration from config.yaml file and create a new mutable Api instance,
        let api_config = ApiConfig::default();
        let auto_login = api_config.auto_login.unwrap_or(false);
        let mut rest_api = match RestApi::new(api_config).await {
            Ok(api) => api,
            Err(e) => panic!("Failed to create and initialize REST API: {}", e),
        };

        if !auto_login {
            let _ = rest_api.client.login();
        }

        Arc::new(rest_api)
    })
    .await
    .clone()
}

/// Read ENV variable RUST_TEST_DELAY_SECONDS and sleep for that many seconds.
fn sleep() {
    let seconds: u64 = std::env::var("RUST_TEST_DELAY")
        .unwrap_or(DEFAULT_TEST_DELAY_SECONDS.to_string())
        .parse::<u64>()
        .unwrap_or(DEFAULT_TEST_DELAY_SECONDS);

    std::thread::sleep(std::time::Duration::from_secs(seconds));
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// INTEGRATION TESTS
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Force this test to run first by using aaa_ prefix to ensure the API is
/// properly initialized before running other tests.
#[tokio::test]
async fn aaa_rest_api_is_properly_initialized() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;
    println!("API instance: {:?}", api);

    // First check if auth headers are set.
    assert!(api.client.auth_headers.is_some());

    // Then check auth tokens are set and have the correct format for the configured session version.
    if let Some(session_version) = api.client.config.session_version.as_ref() {
        match session_version {
            1 | 2 => {
                assert!(api
                    .client
                    .auth_headers
                    .as_ref()
                    .unwrap()
                    .contains_key("cst"));
                assert!(api
                    .client
                    .auth_headers
                    .as_ref()
                    .unwrap()
                    .contains_key("x-security-token"));

                let cst_value = api
                    .client
                    .auth_headers
                    .as_ref()
                    .unwrap()
                    .get("cst")
                    .unwrap()
                    .to_str()
                    .unwrap();
                let re = Regex::new(r"^[a-fA-F0-9]{69}$").unwrap();
                assert!(re.is_match(cst_value));

                let security_token_value = api
                    .client
                    .auth_headers
                    .as_ref()
                    .unwrap()
                    .get("x-security-token")
                    .unwrap()
                    .to_str()
                    .unwrap();
                let re = Regex::new(r"^[a-fA-F0-9]{69}$").unwrap();
                assert!(re.is_match(security_token_value));
            }
            3 => {
                assert!(api
                    .client
                    .auth_headers
                    .as_ref()
                    .unwrap()
                    .contains_key("authorization"));

                let authorization_value = api
                    .client
                    .auth_headers
                    .as_ref()
                    .unwrap()
                    .get("authorization")
                    .unwrap()
                    .to_str()
                    .unwrap();
                let re = regex::Regex::new(
                    r"^Bearer [0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$"
                ).unwrap();
                assert!(re.is_match(authorization_value));
            }
            _ => panic!("Invalid session version: {}", session_version),
        }
    } else {
        panic!("Session version is not set in the configuration.");
    }

    sleep();
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// ACCOUNT ENDPOINT INTEGRATION TESTS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn accounts_get_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    let response = match api.accounts_get().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting accounts: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response.0).unwrap()
    );
    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response.1).unwrap()
    );

    sleep();
}

#[tokio::test]
async fn accounts_preferences_get_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    let response = match api.accounts_preferences_get().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting account preferences: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response.0).unwrap()
    );
    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response.1).unwrap()
    );

    sleep();
}

#[tokio::test]
async fn accounts_preferences_put_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    //
    // First get the current account preferences.
    //
    let response_1 = match api.accounts_preferences_get().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting account preferences: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response_1 headers: {}",
        serde_json::to_string_pretty(&response_1.0).unwrap()
    );
    println!(
        "Response_1 body: {}",
        serde_json::to_string_pretty(&response_1.1).unwrap()
    );

    //
    // According to the current account preferences, update the trailing_stops_enabled field.
    //
    let body_1;
    let body_2;
    match response_1.1.trailing_stops_enabled {
        true => {
            body_1 = AccountsPreferencesRequest {
                trailing_stops_enabled: false,
            };
            body_2 = AccountsPreferencesRequest {
                trailing_stops_enabled: true,
            };
        },
        false => {
            body_1 = AccountsPreferencesRequest {
                trailing_stops_enabled: true,
            };
            body_2 = AccountsPreferencesRequest {
                trailing_stops_enabled: false,
            };
        },
    }

    let response_2 = match api.accounts_preferences_put(&body_1).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error updating account preferences: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response_2 headers: {}",
        serde_json::to_string_pretty(&response_2.0).unwrap()
    );
    println!(
        "Response_2 body: {}",
        serde_json::to_string_pretty(&response_2.1).unwrap()
    );

    //
    // Then get the updated account preferences.
    //
    let response_3 = match api.accounts_preferences_get().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting account preferences: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response_3 headers: {}",
        serde_json::to_string_pretty(&response_3.0).unwrap()
    );
    println!(
        "Response_3 body: {}",
        serde_json::to_string_pretty(&response_3.1).unwrap()
    );

    //
    // Update the account preferences back to the original state.
    //
    let response_4 = match api.accounts_preferences_put(&body_2).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error updating account preferences: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response_4 headers: {}",
        serde_json::to_string_pretty(&response_4.0).unwrap()
    );
    println!(
        "Response_4 body: {}",
        serde_json::to_string_pretty(&response_4.1).unwrap()
    );

    //
    // Finally, get the account preferences again to verify that the update was successful.
    //
    let response_5 = match api.accounts_preferences_get().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting account preferences: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response_5 headers: {}",
        serde_json::to_string_pretty(&response_5.0).unwrap()
    );
    println!(
        "Response_5 body: {}",
        serde_json::to_string_pretty(&response_5.1).unwrap()
    );

    //
    // Verify that the original trailing_stops_enabled field differs from the updated field.
    //
    assert_eq!(response_1.1.trailing_stops_enabled, !response_3.1.trailing_stops_enabled);
    //
    // Verify that the trailing_stops_enabled field has the same value as the original account preferences.
    //
    assert_eq!(response_1.1.trailing_stops_enabled, response_5.1.trailing_stops_enabled);

    sleep();
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// CONFIRMS ENDPOINT INTEGRATION TESTS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn confirms_get_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    // TODO: Replace the deal_reference with a valid deal reference.
    let params = ConfirmsRequest {
        deal_reference: "76GAP71HRC2SAQ4B".to_string(),
    };

    let response = match api.confirms_get(params).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting confirms: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response.0).unwrap()
    );
    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response.1).unwrap()
    );

    sleep();
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// HISTORY ENDPOINT INTEGRATION TESTS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn history_activity_get_works() {
    // Initialize the API instance.
    let api = get_or_init_rest_api().await;

    let params = ActivityHistoryRequest {
        // 10 years ago.
        from: chrono::Utc::now().naive_utc() - chrono::Duration::days(3650),
        to: None,
        detailed: None,
        deal_id: None,
        filter: None,
        page_size: Some(10),
    };

    // Test without parameters.
    let response = match api.history_activity_get(params).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting activity history: {:?}", e);
            panic!("Test failed due to an error.");
        }
    };

    // Print the response for manual verification.
    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response.0).unwrap()
    );
    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response.1).unwrap()
    );
}

#[tokio::test]
async fn history_transactions_get_works() {
    // Initialize the API instance.
    let api = get_or_init_rest_api().await;

    let params = TransactionHistoryRequest {
        r#type: None,
        // 10 years ago.
        from: chrono::Utc::now().naive_utc() - chrono::Duration::days(3650),
        to: None,
        max_span_seconds: None,
        page_size: Some(10),
        page_number: None,
    };

    // Test without parameters.
    let response = match api.history_transactions_get(params).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting transaction history: {:?}", e);
            panic!("Test failed due to an error.");
        }
    };

    // Print the response for manual verification.
    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response.0).unwrap()
    );
    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response.1).unwrap()
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// POSITIONS ENDPOINT INTEGRATION TESTS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn positions_get_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    let response = match api.positions_get().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting list of positions: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response.0).unwrap()
    );
    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response.1).unwrap()
    );

    sleep();
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// SESSION ENDPOINT INTEGRATION TESTS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn session_get_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    // Test with no params.
    let response = match api.session_get(None).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting session details with no params: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response.0).unwrap()
    );
    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response.1).unwrap()
    );

    // Test with params.
    let params = SessionDetailsRequest {
        fetch_session_tokens: true,
    };

    let response = match api.session_get(Some(params)).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting session details with params: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response.0).unwrap()
    );
    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response.1).unwrap()
    );

    sleep();
}

#[tokio::test]
async fn session_put_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    // If config account_number_test is not set, then skip this test.
    if api.config.account_number_test.is_none() {
        println!("Skipping test because account_number_test is not set in configuration file.");
        return;
    }

    let new_account_number = match api.config.account_number_test.clone() {
        Some(account_number) => account_number,
        None => {
            println!("No test account number is set in the configuration.");
            panic!("Test failed due to error.");
        }
    };

    let body = AccountSwitchRequest {
        account_id: new_account_number.clone(),
        default_account: None,
    };

    let response = match api.session_put(&body).await {
        Ok(response) => response,
        Err(e) => {
            println!(
                "Error switching session to account '{}': {:?}",
                new_account_number, e
            );
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response.0).unwrap()
    );
    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response.1).unwrap()
    );

    sleep();
}

#[tokio::test]
async fn session_encryption_key_get_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    let response = match api.session_encryption_key_get().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting session encryption key: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response.0).unwrap()
    );
    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response.1).unwrap()
    );

    let encryption_key = response.1.encryption_key.as_bytes();
    let decoded_encryption_key = Base64Display::new(encryption_key, &STANDARD);
    println!("Decoded encryption key: {}", decoded_encryption_key);

    sleep();
}

#[tokio::test]
async fn session_refresh_token_post_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    // If config session_version is not 3, then skip this test.
    if api.client.config.session_version.unwrap_or(0) != 3 {
        println!("Skipping test because session_version is not 3 in configuration file.");
        return;
    }

    let body = SessionRefreshTokenRequest {
        refresh_token: api.client.refresh_token.as_ref().unwrap().clone(),
    };

    println!("Refresh token: {:?}", body.refresh_token);
    println!("Auth headers: {:?}", api.client.auth_headers.as_ref().unwrap());

    let response: (Value, SessionRefreshTokenResponse) = match api.session_refresh_token_post(&body).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error refreshing session token: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response.0).unwrap()
    );
    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response.1).unwrap()
    );

    sleep();
}

/// Force this test to run last by using zzz_ prefix to ensure the session
/// is not deleted before running other tests.
#[tokio::test]
async fn zzz_session_delete_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    let response = match api.session_delete().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting session details: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response.0).unwrap()
    );

    sleep();
}
