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

        // Force tests are performed in the DEMO environment.
        rest_api.client.config.execution_environment = ExecutionEnvironment::Demo;

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
    println!("Getting the current account preferences...");
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

    sleep();

    //
    // According to the current account preferences, update the trailing_stops_enabled field.
    //
    println!("Updating the account preferences...");
    let body_1;
    let body_2;
    match response_1.1.trailing_stops_enabled {
        true => {
            body_1 = AccountsPreferencesPutRequest {
                trailing_stops_enabled: false,
            };
            body_2 = AccountsPreferencesPutRequest {
                trailing_stops_enabled: true,
            };
        },
        false => {
            body_1 = AccountsPreferencesPutRequest {
                trailing_stops_enabled: true,
            };
            body_2 = AccountsPreferencesPutRequest {
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

    sleep();

    //
    // Then get the updated account preferences.
    //
    println!("Getting the updated account preferences...");
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

    sleep();

    //
    // Update the account preferences back to the original state.
    //
    println!("Updating the account preferences back to the original state...");
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

    sleep();

    //
    // Finally, get the account preferences again to verify that the update was successful.
    //
    println!("Getting the account preferences again...");
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
// HISTORY ENDPOINT INTEGRATION TESTS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn history_activity_get_works() {
    // Initialize the API instance.
    let api = get_or_init_rest_api().await;

    let params = ActivityHistoryGetRequest {
        // 10 years ago.
        from: chrono::Utc::now().naive_utc() - chrono::Duration::try_days(3650).unwrap(),
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

    let params = TransactionHistoryGetRequest {
        r#type: None,
        // 10 years ago.
        from: chrono::Utc::now().naive_utc() - chrono::Duration::try_days(3650).unwrap(),
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
// MARKETS ENDPOINT INTEGRATION TESTS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn marketnavigation_get_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    let response = match api.marketnavigation_get().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting markets: {:?}", e);
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
// POSITIONS ENDPOINT INTEGRATION TESTS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn positions_flow_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    //
    // First create a new position.
    //
    println!("Creating a new position...");
    let position_request = PositionPostRequest {
        currency_code: "EUR".to_string(),
        deal_reference: None,
        direction: Direction::Buy,
        epic: "IX.D.DAX.IFMM.IP".to_string(),
        expiry: "-".to_string(), // "-" for no expiry.
        force_open: true,
        guaranteed_stop: false,
        level: None,
        limit_distance: None,
        limit_level: None,
        order_type: OrderType::Market,
        quote_id: None,
        size: 1.0,
        stop_distance: None,
        stop_level: None,
        time_in_force: None,
        trailing_stop: None,
        trailing_stop_increment: None,
    };

    let response_1 = match api.position_post(position_request).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error creating a new position: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response_1.0).unwrap()
    );

    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response_1.1).unwrap()
    );

    let deal_reference = response_1.1.deal_reference.clone();

    //
    // Get the trade confirmation for the new position.
    //
    println!("Getting trade confirmation for the new position...");
    let response_2 = match api.confirms_get(ConfirmsGetRequest {
        deal_reference: deal_reference.clone(),
    }).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting trade confirmation for the new position: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response_2.0).unwrap()
    );

    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response_2.1).unwrap()
    );

    let deal_id = response_2.1.deal_id.clone();
    let position_level = response_2.1.level.unwrap();
    println!("Position level: {}", position_level);

    //
    // Get the list of open positions.
    //
    println!("Getting list of open positions...");
    let response_3 = match api.positions_get().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting list of positions: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response_3.0).unwrap()
    );

    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response_3.1).unwrap()
    );

    sleep();

    //
    // Get details of the new position.
    //
    println!("Getting details of the position...");
    let params = PositionGetRequest { deal_id: deal_id.clone() };

    let response_4 = match api.position_get(params).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting position '{}': {:?}", deal_id, e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response_4.0).unwrap()
    );

    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response_4.1).unwrap()
    );

    sleep();

    //
    // Update the position.
    //
    println!("Updating the position...");
    let position_update_request = PositionPutRequest {
        guaranteed_stop: Some(true),
        limit_level: Some(position_level + 100.0),
        stop_level: Some(position_level - 50.0),
        trailing_stop: None,
        trailing_stop_distance: None,
        trailing_stop_increment: None,
    };

    let response_5 = match api.position_put(position_update_request, deal_id.clone()).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error updating the new position: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response_5.0).unwrap()
    );

    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response_5.1).unwrap()
    );

    sleep();

    //
    // Close the position.
    //
    println!("Closing the position...");
    let position_close_request = PositionDeleteRequest {
        deal_id: Some(deal_id),
        direction: Some(Direction::Sell),
        epic: None,
        expiry: Some("-".to_string()), // "-" for no expiry.
        level: None,
        order_type: Some(OrderType::Market),
        quote_id: None,
        size: 1.0,
        time_in_force: None,
    };

    let response_6 = match api.position_delete(position_close_request).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error closing the new position: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response_6.0).unwrap()
    );

    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response_6.1).unwrap()
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
    let params = SessionDetailsGetRequest {
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

    let body = AccountSwitchPutRequest {
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

    let body = SessionRefreshTokenPostRequest {
        refresh_token: api.client.refresh_token.as_ref().unwrap().clone(),
    };

    println!("Refresh token: {:?}", body.refresh_token);
    println!("Auth headers: {:?}", api.client.auth_headers.as_ref().unwrap());

    let response: (Value, SessionRefreshTokenPostResponse) = match api.session_refresh_token_post(&body).await {
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

////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// WORKINGORDERS ENDPOINT INTEGRATION TESTS.
//
////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn workingorders_flow_works() {
    // Get the API instance.
    let api = get_or_init_rest_api().await;

    //
    // Create a new working order.
    //
    println!("Creating a new working order...");
    let working_order_request = WorkingOrderPostRequest {
        currency_code: "EUR".to_string(),
        deal_reference: None,
        direction: Direction::Buy,
        epic: "IX.D.DAX.IFMM.IP".to_string(),
        expiry: "-".to_string(), // "-" for no expiry.
        force_open: Some(true),
        good_till_date: None,
        guaranteed_stop: false,
        level: 10000.0,
        limit_distance: None,
        limit_level: None,
        size: 1.0,
        stop_distance: None,
        stop_level: None,
        time_in_force: WorkingOrderTimeInForce::GoodTillCancelled,
        r#type: WorkingOrderType::Limit,
    };

    let response_1 = match api.workingorders_post(&working_order_request).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error creating a new working order: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response_1.0).unwrap()
    );

    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response_1.1).unwrap()
    );

    let deal_reference = response_1.1.deal_reference.clone();

    sleep();

    //
    // Get the trade confirmation for the new position.
    //
    println!("Getting trade confirmation for the new working order...");
    let response_2 = match api.confirms_get(ConfirmsGetRequest {
        deal_reference: deal_reference.clone(),
    }).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting trade confirmation for the new working order: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response_2.0).unwrap()
    );

    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response_2.1).unwrap()
    );

    let deal_id = response_2.1.deal_id.clone();
    let working_order_level;
    if let Some(level) = response_2.1.level {
        working_order_level = level;
    } else {
        working_order_level = 18500.0;
    }
    println!("Working order level: {}", working_order_level);

    sleep();

    //
    // Get the list of working orders.
    //
    println!("Getting list of working orders...");
    let response_3 = match api.workingorders_get().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting list of working orders: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response_3.0).unwrap()
    );

    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response_3.1).unwrap()
    );

    sleep();

    //
    // Update the working order.
    //
    println!("Updating the new working order...");
    let working_order_update_request = WorkingOrderPutRequest {
        good_till_date: None,
        guaranteed_stop: None,
        level: working_order_level + 100.0,
        limit_distance: None,
        limit_level: None,
        stop_distance: None,
        stop_level: None,
        time_in_force: WorkingOrderTimeInForce::GoodTillCancelled,
        r#type: WorkingOrderType::Limit,
    };

    let response_4 = match api.workingorders_put(&working_order_update_request, deal_id.clone()).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error updating the new working order: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response_4.0).unwrap()
    );

    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response_4.1).unwrap()
    );

    sleep();

    //
    // Get the list of working orders.
    //
    println!("Getting list of working orders...");
    let response_5 = match api.workingorders_get().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting list of working orders: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response_5.0).unwrap()
    );

    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response_5.1).unwrap()
    );

    sleep();

    //
    // Delete the working order.
    //
    println!("Deleting the new working order...");
    let response_6 = match api.workingorders_delete(deal_id.clone()).await {
        Ok(response) => response,
        Err(e) => {
            println!("Error deleting the new working order: {:?}", e);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Response headers: {}",
        serde_json::to_string_pretty(&response_6.0).unwrap()
    );

    println!(
        "Response body: {}",
        serde_json::to_string_pretty(&response_6.1).unwrap()
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
