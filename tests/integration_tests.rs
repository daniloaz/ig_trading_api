use ig_trading_api::common::*;
use ig_trading_api::rest_api::*;

async fn login_v2() -> Result<RestApi, ApiError> {
    // Load the configuration and create a new mutable Api instance,
    // as it will be modified with the CST and X-SECURITY-TOKEN values.
    let api_config = ApiConfig::default();
    let mut api = RestApi::new(api_config);

    let _ = match api.login_v2().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error logging in: {}", e.message);
            panic!("Test failed due to error.");
        }
    };

    Ok(api)
}

#[tokio::test]
async fn get_session_works() {
    // Initialize a logged in API instance.
    let api = login_v2().await.unwrap();

    // Then get the session details.
    let response = match api.get_session().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error getting session details: {}", e.message);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Session details: {}",
        serde_json::to_string_pretty(&response).unwrap()
    );
}

#[tokio::test]
async fn login_v2_works() {
    // Load the configuration and create a new mutable Api instance,
    // as it will be modified with the CST and X-SECURITY-TOKEN values.
    let api_config = ApiConfig::default();
    let mut api = RestApi::new(api_config);

    println!("Testing login with username: {}", api.config.username);

    let response = match api.login_v2().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error logging in: {}", e.message);
            panic!("Test failed due to error.");
        }
    };

    println!(
        "Login response: {}",
        serde_json::to_string_pretty(&response).unwrap()
    );
    println!("Api instance: {:?}", api);
}

#[tokio::test]
async fn logout_works() {
    // Initialize a logged in API instance.
    let api = login_v2().await.unwrap();

    // Then log out from the API.
    let _ = match api.logout().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error logging out from current session: {}", e.message);
            panic!("Test failed due to error.");
        }
    };
}
