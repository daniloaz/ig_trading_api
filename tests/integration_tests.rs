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
