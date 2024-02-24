use ig_trading_api::common::*;
use ig_trading_api::rest_api::*;
use regex::Regex;
use std::sync::Arc;
use tokio::sync::OnceCell;

static API: OnceCell<Arc<RestApi>> = OnceCell::const_new();
static DEFAULT_TEST_DELAY_SECONDS: u64 = 10;

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
                todo!("Implement tests for session version 3.");
            }
            _ => panic!("Invalid session version: {}", session_version),
        }
    } else {
        panic!("Session version is not set in the configuration.");
    }

    sleep();
}

#[tokio::test]
async fn get_session_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    let response = match api.get_session(None).await {
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
    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response.1).unwrap()
    );

    sleep();
}

/// Force this test to run last by using zzz_ prefix to ensure the session
/// is not deleted before running other tests.
#[tokio::test]
async fn zzz_delete_session_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    let response = match api.delete_session().await {
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